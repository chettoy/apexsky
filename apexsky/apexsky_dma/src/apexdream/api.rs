use dataview::Pod;
use intptr::IntPtr64 as Ptr;
use std::{fmt, mem};
use tracing::instrument;

use crate::mem::ApexMem;

#[derive(Debug)]
pub struct Error;

#[derive(Debug)]
pub struct Api<'b, 'c> {
    pub apex_mem: ApexMem<'b, 'c>,
}

impl<'b, 'c> Api<'b, 'c> {
    /// Standard log function.
    #[instrument(skip_all)]
    pub fn log(&mut self, args: impl fmt::Display) {
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
    pub fn vm_read<T: Pod>(&mut self, ptr: Ptr<T>) -> Result<T, Error> {
        unsafe {
            // Yes yes but this isn't easy to fix...
            #[allow(deprecated)]
            let mut dest: T = mem::uninitialized();
            let result = {
                let dest = dataview::bytes_mut(&mut dest);
                self.apex_mem.read_memory(ptr.into_raw(), dest)
            };
            if result >= 0 {
                Ok(dest)
            } else {
                #[cfg(feature = "debug_api")]
				self.log(fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_read("{ptr}"): "{result}));
                Err(Error)
            }
        }
    }

    /// Reads memory into the destination from the process.
    #[cfg_attr(feature = "debug_api", track_caller)]
    #[inline]
    pub fn vm_read_into<T: Pod + ?Sized>(
        &mut self,
        ptr: Ptr<T>,
        dest: &mut T,
    ) -> Result<(), Error> {
        let result = {
            let dest = dataview::bytes_mut(dest);
            self.apex_mem.read_memory(ptr.into_raw(), dest)
        };
        if result >= 0 {
            Ok(())
        } else {
            #[cfg(feature = "debug_api")]
			self.log(fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_read_into("{ptr}"): "{result}));
            Err(Error)
        }
    }

    /// Gathers memory from the process.
    /// This routine is optimized for reading small pieces of large objects.
    #[cfg_attr(feature = "debug_api", track_caller)]
    #[inline]
    pub fn vm_gatherd<'a, T: Pod>(
        &mut self,
        ptr: Ptr,
        _size: u32,
        indices: &'a mut T,
    ) -> Result<&'a T, Error> {
        let view_mut = dataview::DataView::from_mut(indices);
        let view_mut = view_mut.slice_mut::<u32>(0, view_mut.tail_len::<u32>(0));
        let result = self.gather_memory(ptr.into_raw(), view_mut);
        if result >= 0 {
            Ok(indices)
        } else {
            #[cfg(feature = "debug_api")]
			self.log(fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_gatherd("{ptr}"): "{result}));
            Err(Error)
        }
    }

    fn gather_memory(&mut self, base_address: u64, indices: &mut [u32]) -> i32 {
        let mut buf = [0u8; 0x1000];

        // Keep track of indices read within reasonable limit
        if indices.len() >= 128 {
            return -1;
        }
        let mut read_mask = 0u128;

        // For every index
        let mut success = false;
        for i in 0..indices.len() {
            if read_mask & (1u128 << i) == 0 {
                let virtual_address = (base_address + indices[i] as u64) & !0xfff;
                let temp = if self.apex_mem.read_memory(virtual_address, &mut buf) >= 0 {
                    // If a single read was succesful the whole read is successful
                    success = true;
                    Some(&buf)
                } else {
                    None
                };

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

        if success {
            0
        } else {
            -1
        }
    }

    /// Reads bytes to be interpreted as a c-string.
    pub fn vm_read_cstr<'a>(
        &mut self,
        ptr: Ptr<[u8]>,
        buf: &'a mut [u8],
    ) -> Result<&'a str, Error> {
        self.vm_read_into(ptr, buf)?;
        crate::apexdream::base::from_utf8_buf(buf).ok_or(Error)
    }

    /// Writes memory into the process.
    #[cfg_attr(feature = "debug_api", track_caller)]
    #[inline]
    pub fn vm_write<T: Pod + ?Sized>(&mut self, ptr: Ptr<T>, data: &T) -> Result<(), Error> {
        let result = {
            let data = dataview::bytes(data);
            self.apex_mem.write_memory(ptr.into_raw(), data)
        };
        if result >= 0 {
            Ok(())
        } else {
            #[cfg(feature = "debug_api")]
			self.log(fmtools::fmt!("error: "{std::panic::Location::caller()}" vm_write("{ptr}"): "{result}));
            Err(Error)
        }
    }
}
