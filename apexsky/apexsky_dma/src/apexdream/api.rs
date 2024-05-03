use dataview::Pod;
use intptr::IntPtr64 as Ptr;
use obfstr::obfstr as s;
use std::{fmt, mem};
use tracing::instrument;

use crate::workers::access::{AccessType, PendingAccessRequest, MemApi};

#[derive(Debug)]
pub struct Api {
    pub apex_base: intptr::IntPtr64,
    pub mem_access: MemApi,
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

    /// Reads memory from the process.
    #[instrument]
    #[cfg_attr(feature = "debug_api", track_caller)]
    #[inline]
    pub async fn vm_read<T: Pod>(&self, ptr: Ptr<T>) -> anyhow::Result<T> {
        let mut dest: T = unsafe { mem::MaybeUninit::zeroed().assume_init() };
        let result = {
            let dest = dataview::bytes_mut(&mut dest);
            AccessType::mem_read(ptr.into_raw(), dest.len(), 0)
                .dispatch(&self.mem_access)
                .await?
                .await?
                .map(|data| dest.copy_from_slice(&data))
        };
        result.map(|_| dest).map_err(|e| {
            #[cfg(feature = "debug_api")]
            self.log(
                fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_read("{ptr}"): "{result}),
            );
            e
        })
    }

    /// Reads memory into the destination from the process.
    #[cfg_attr(feature = "debug_api", track_caller)]
    #[inline]
    pub async fn vm_read_into<T: Pod + ?Sized>(
        &self,
        ptr: Ptr<T>,
        dest: &mut T,
    ) -> anyhow::Result<()> {
        let result = {
            let dest = dataview::bytes_mut(dest);
            AccessType::mem_read(ptr.into_raw(), dest.len(), 0)
                .dispatch(&self.mem_access)
                .await?
                .await?
                .map(|data| dest.copy_from_slice(&data))
        };
        result.map_err(|e| {
            #[cfg(feature = "debug_api")]
			self.log(fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_read_into("{ptr}"): "{result}));
            e
        })
    }

    /// Gathers memory from the process.
    /// This routine is optimized for reading small pieces of large objects.
    #[cfg_attr(feature = "debug_api", track_caller)]
    #[inline]
    pub async fn vm_gatherd<'a, T: Pod>(
        &self,
        ptr: Ptr,
        _size: u32,
        indices: &'a mut T,
    ) -> anyhow::Result<&'a T> {
        let view_mut = dataview::DataView::from_mut(indices);
        let view_mut = view_mut.slice_mut::<u32>(0, view_mut.tail_len::<u32>(0));
        self.gather_memory(ptr.into_raw(), view_mut)
            .await
            .map(|_| &*indices)
            .map_err(|e| {
                #[cfg(feature = "debug_api")]
			self.log(fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_gatherd("{ptr}"): "{result}));
                e
            })
    }

    async fn gather_memory(&self, base_address: u64, indices: &mut [u32]) -> anyhow::Result<()> {
        let mut buf = [0u8; 0x1000];

        // Keep track of indices read within reasonable limit
        if indices.len() >= 128 {
            anyhow::bail!("{}", s!("227f1a4a-6c74-47bc-a1ab-a3df872c6efc"));
        }
        let mut read_mask = 0u128;

        // For every index
        for i in 0..indices.len() {
            if read_mask & (1u128 << i) == 0 {
                let virtual_address = (base_address + indices[i] as u64) & !0xfff;
                let temp = AccessType::mem_read(virtual_address, buf.len(), 0)
                    .with_priority(0)
                    .dispatch(&self.mem_access)
                    .await?
                    .await?
                    .ok()
                    .and_then(|data| {
                        buf.copy_from_slice(&data);
                        Some(&buf)
                    });

                // Read all indices in the page
                for j in i..indices.len() {
                    if read_mask & (1u128 << j) == 0 {
                        let index_address = base_address + indices[j] as u64;
                        if index_address >= virtual_address
                            && index_address < virtual_address + 0x1000
                        {
                            // Mark the index as read
                            read_mask |= 1u128 << j;

                            // Try to read the index
                            // Write zero if underlying page failed to read or index straddling 4K boundary
                            let index_offset = (index_address - virtual_address) as usize;
                            indices[j] = temp
                                .and_then(|temp| temp.get(index_offset..index_offset + 4))
                                .map(|dword| {
                                    u32::from_ne_bytes([dword[0], dword[1], dword[2], dword[3]])
                                })
                                .unwrap_or(0);
                        }
                    }
                }
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
    #[cfg_attr(feature = "debug_api", track_caller)]
    #[inline]
    pub async fn vm_write<T: Pod + ?Sized>(&self, ptr: Ptr<T>, data: &T) -> anyhow::Result<()> {
        AccessType::mem_write_typed(ptr.into_raw(), data, 0)
            .dispatch(&self.mem_access)
            .await?
            .await?
            .map_err(|e| {
                #[cfg(feature = "debug_api")]
			self.log(fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_write("{ptr}"): "{result}));
                e
            })
    }
}
