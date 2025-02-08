use core::slice;
use obfstr::obfstr as s;
use std::{collections::HashSet, fmt, mem};
use tracing::instrument;
use zerocopy::{FromBytes, Immutable, IntoBytes};

use ohosky_api::{common::dmalib::IMemAccess, skydream::dmalib};

use super::sdk::Ptr;

#[derive(Debug, Clone)]
pub struct Api {
    pub apex_base: Ptr,
    pub mem_access: dmalib::MemAccess,
    pub req_id: usize,
}

impl Api {
    /// Standard log function.
    #[instrument(skip_all)]
    pub fn log(&self, args: impl fmt::Display) {
        tracing::debug!("{}", format_args!("{}", args))
    }

    /// Visualize the args in a scope.
    ///
    /// The `args` is some html that should replace the previous contents of `scope`.
    #[instrument(skip_all)]
    pub fn visualize(&mut self, scope: &str, args: impl fmt::Display) {
        tracing::debug!(?scope, "{}", format_args!("{}", args))
    }

    pub fn clone_new(&self, req_id: usize) -> Self {
        Self {
            apex_base: self.apex_base,
            mem_access: self.mem_access.clone(),
            req_id,
        }
    }

    /// Reads memory from the process.
    #[inline]
    pub async fn vm_read<T: FromBytes>(&self, ptr: Ptr<T>) -> anyhow::Result<T> {
        self.mem_access
            .read_raw(
                ptr.into_raw(),
                size_of::<T>(),
                dmalib::PRIO_LOW,
                self.req_id,
            )
            .await
            .map(|bytes| T::read_from_bytes(&bytes).unwrap())
            .inspect_err(|e| {
                tracing::debug!(?ptr, ?e);
            })
    }

    /// Reads memory into the destination from the process.
    #[inline]
    pub async fn vm_read_into<T: FromBytes + IntoBytes + ?Sized>(
        &self,
        ptr: Ptr<T>,
        dest: &mut T,
    ) -> anyhow::Result<()> {
        let dest = dest.as_mut_bytes();
        self.mem_access
            .read_raw(ptr.into_raw(), dest.len(), dmalib::PRIO_LOW, self.req_id)
            .await
            .map(|bytes| dest.copy_from_slice(&bytes))
            .inspect_err(|e| tracing::debug!(?ptr, ?e))
    }

    /// Gathers memory from the process.
    /// This routine is optimized for reading small pieces of large objects.
    #[inline]
    pub async fn vm_gatherd<'a, T: FromBytes + IntoBytes>(
        &self,
        ptr: Ptr,
        _size: u32,
        ignore_zero_offset: bool,
        indices: &'a mut T,
    ) -> anyhow::Result<&'a T> {
        fn is_aligned<U>(ptr: *const U) -> bool {
            let addr: usize = unsafe { mem::transmute(ptr) };
            addr % mem::align_of::<U>() == 0
        }

        /// Gets an aligned mutable slice into the view
        #[inline]
        fn try_slice_mut<U>(bytes: &mut [u8], offset: usize, len: usize) -> Option<&mut [U]> {
            let index = offset..offset + usize::checked_mul(len, size_of::<U>())?;
            let bytes = bytes.get_mut(index)?;
            let unaligned_ptr = bytes.as_mut_ptr() as *mut U;
            if !is_aligned(unaligned_ptr) {
                return None;
            }
            unsafe { Some(slice::from_raw_parts_mut(unaligned_ptr, len)) }
        }

        let view_mut = indices.as_mut_bytes();
        let view_mut =
            try_slice_mut::<u32>(view_mut, 0, view_mut.len() / size_of::<u32>()).unwrap();

        self.gather_memory(ptr.into_raw(), ignore_zero_offset, view_mut)
            .await
            .map(|_| &*indices)
            .inspect_err(|e| tracing::debug!(?ptr, ?e))
    }

    async fn gather_memory(
        &self,
        base_address: u64,
        ignore_zero_offset: bool,
        indices: &mut [u32],
    ) -> anyhow::Result<()> {
        // Keep track of indices read within reasonable limit
        let len = indices.len();
        if len >= 128 {
            anyhow::bail!("{}", s!("227f1a4a-6c74-47bc-a1ab-a3df872c6efc"));
        }
        let mut read_mask = 0u128;

        let virtual_address: Vec<Option<u64>> = if ignore_zero_offset {
            indices
                .iter()
                .map(|&i| (i != 0).then_some(base_address + i as u64))
                .collect()
        } else {
            indices
                .iter()
                .map(|&i| Some(base_address + i as u64))
                .collect()
        };

        let page_address: HashSet<u64> = virtual_address
            .iter()
            .filter_map(|va| va.as_ref())
            .map(|&addr| addr & !0xfff)
            .collect();

        let mut read_pages: Vec<(u64, usize, Option<_>)> = page_address
            .into_iter()
            .map(|va| (va, 0x1000, None))
            .collect();

        self.mem_access
            .read_raw_list(&mut read_pages, dmalib::PRIO_LOW, self.req_id)
            .await?;

        for (page_addr, _len, data) in read_pages {
            if data.is_none() {
                anyhow::bail!("{}{:x}", s!("err read page 0x"), page_addr);
            }

            // For every index
            for i in 0..len {
                if read_mask & (1u128 << i) != 0 {
                    continue;
                }

                let Some(va) = virtual_address[i] else {
                    // Set data to 0 if addr is 0
                    assert_eq!(indices[i], 0);
                    // Mark the index as read
                    read_mask |= 1u128 << i;
                    continue;
                };

                if va & !0xfff != page_addr {
                    continue;
                }

                // Mark the index as read
                read_mask |= 1u128 << i;

                // Try to read the index
                // Write zero if underlying page failed to read or index straddling 4K boundary
                let index_offset = (va & 0xfff) as usize;
                indices[i] = data
                    .as_ref()
                    .and_then(|temp| temp.get(index_offset..index_offset + 4))
                    .map(|dword| u32::from_ne_bytes([dword[0], dword[1], dword[2], dword[3]]))
                    .unwrap_or(0);
            }
        }

        Ok(())
    }

    /// Reads bytes to be interpreted as a c-string.
    pub async fn vm_read_cstr<'a>(
        &self,
        ptr: Ptr<[u8]>,
        buf: &'a mut [u8],
    ) -> anyhow::Result<&'a str> {
        self.vm_read_into(ptr, buf).await?;
        crate::apexdream::base::from_utf8_buf(buf).ok_or(anyhow::anyhow!("{}", s!("from_utf8_buf")))
    }

    /// Writes memory into the process.
    #[inline]
    pub async fn vm_write<T: IntoBytes + Immutable>(
        &self,
        ptr: Ptr<T>,
        data: &T,
    ) -> anyhow::Result<()> {
        self.mem_access
            .write(ptr.into_raw(), data, dmalib::PRIO_LOW, self.req_id)
            .await
            .inspect_err(|e| tracing::debug!(?ptr, ?e))
    }
}
