use crate::common::dmalib::{
    DmalibAccessTarget, IMemAccess, PRIO_HIGH, PRIO_LOW, PRIO_PASSIVE, PRIO_PREEMPT,
};

use super::skyapi::MemAccess;

#[cxx::bridge]
mod ffi {
    /// ohosky_dmalib CXX API
    struct Memory {
        handle: u64,
    }

    /// Access priority levels
    enum AccessPriority {
        PRIO_PASSIVE = -1,
        PRIO_LOW = 0,
        PRIO_HIGH = 1,
        PRIO_PREEMPT = 0x10,
    }

    /// Result structure of signature finding operations.
    struct FindSignatureResult {
        found: bool,
        address: u64,
        data: Vec<u8>,
    }

    struct BatchReadItem<'a> {
        address: u64,
        out: &'a mut [u8],
    }

    extern "Rust" {

        /// @brief Initializes the DMA Lib handle. This must be called before any DMA operations.
        ///
        /// @param process_name The name of the process.
        /// @param check_time_date_stamp The timedatestamp to verify against the target process.
        /// @param speed_test A flag to print the speed test results after initialization.
        /// @param cache_phys_addr A flag for operations using cached physical address.
        /// @exception Will return an error if initialization fails.
        fn init(
            self: &mut Memory,
            process_name: &str,
            check_time_date_stamp: u32,
            speed_test: bool,
            cache_phys_addr: bool,
        ) -> Result<()>;

        /// @brief Retrieves the module section base address of the process.
        ///
        /// @return The base address of the process as uint64 if avaliable or 0 if not avaliable.
        /// @exception Will return an error if the base address cannot be retrieved.
        fn get_base_addr(self: &Memory) -> Result<u64>;

        /// @brief Reads memory from the process.
        ///
        /// @param address The address to read from.
        /// @param size The number of bytes to read.
        /// @param priority The priority level for this operation.
        ///
        /// @return A buffer containing the data read from memory.
        /// @exception Will return an error if the read operation fails.
        fn read_raw(self: &Memory, address: u64, size: usize, priority: i32) -> Result<Vec<u8>>;

        /// @brief Reads memory from the process into a buffer.
        ///
        /// @param address The address to read from.
        /// @param out The buffer to read into.
        /// @param priority The priority level for this operation.
        ///
        /// @exception Will return an error if the read operation fails.
        fn read_raw_into(self: &Memory, address: u64, out: &mut [u8], priority: i32) -> Result<()>;

        /// @brief Reads memory from the process in a batch.
        ///
        /// @param batch A list of BatchReadItem structs containing the address and buffer to read into.
        /// @param priority The priority level for this operation.
        ///
        /// @return The number of successful reads.
        fn batch_read(self: &Memory, batch: Vec<BatchReadItem>, priority: i32) -> usize;

        /// @brief Writes memory to the process.
        ///
        /// @param address The address to write to.
        /// @param data The data to write.
        /// @param priority The priority level for this operation.
        ///
        /// @exception Will return an error if the write operation fails.
        fn write_raw(self: &Memory, address: u64, data: &[u8], priority: i32) -> Result<()>;

        /// @brief Dump the process memory, requires a valid PE header.
        ///
        /// @return A buffer containing the dumped memory data.
        /// @exception Will return an error if the memory dump fails.
        fn dump_memory(self: &Memory) -> Result<Vec<u8>>;

        /// @brief Scans the process for a specified signature.
        ///
        /// @param signature The signature to search for, e.g., \"48 ? ? ?\".
        /// @param range_start The starting address of the scan range.
        /// @param range_end The ending address of the scan range.
        ///
        /// @return A FindSignatureResult struct containing the result of the scan.
        /// @exception Will return an error if the scan fails.
        fn find_signature(
            self: &Memory,
            signature: &str,
            range_start: u64,
            range_end: u64,
        ) -> Result<FindSignatureResult>;
    }
}

const _: () = assert!(ffi::AccessPriority::PRIO_HIGH.repr as i32 == PRIO_HIGH);
const _: () = assert!(ffi::AccessPriority::PRIO_LOW.repr as i32 == PRIO_LOW);
const _: () = assert!(ffi::AccessPriority::PRIO_PASSIVE.repr as i32 == PRIO_PASSIVE);
const _: () = assert!(ffi::AccessPriority::PRIO_PREEMPT.repr as i32 == PRIO_PREEMPT);

impl ffi::Memory {
    fn init(
        &mut self,
        process_name: &str,
        check_time_date_stamp: u32,
        speed_test: bool,
        cache_phys_addr: bool,
    ) -> anyhow::Result<()> {
        let mem = MemAccess::open(&DmalibAccessTarget {
            target_process_name: process_name.to_owned(),
            override_module_base: None,
            check_time_date_stamp: match check_time_date_stamp {
                0 => None,
                n => Some(n),
            },
            speed_test,
            cache_phys_addr,
        })?;
        self.handle = mem.into_handle();
        Ok(())
    }

    fn get_base_addr(&self) -> anyhow::Result<u64> {
        let mem = MemAccess::from_handle(self.handle);
        mem.get_baseaddr_blocking(PRIO_HIGH)
            .map(Option::unwrap_or_default)
    }

    fn read_raw(&self, addr: u64, size: usize, priority: i32) -> anyhow::Result<Vec<u8>> {
        let mem = MemAccess::from_handle(self.handle);
        let ret = mem.read_raw_blocking(addr, size, priority, 0)?;
        Ok(ret.into())
    }

    fn read_raw_into(&self, addr: u64, out: &mut [u8], priority: i32) -> anyhow::Result<()> {
        let mem = MemAccess::from_handle(self.handle);
        mem.read_raw_into_blocking(addr, out, priority, 0)
    }

    fn batch_read(&self, batch: Vec<ffi::BatchReadItem>, priority: i32) -> usize {
        let mem = MemAccess::from_handle(self.handle);

        let (mut read_list, out_list): (Vec<_>, Vec<&mut [u8]>) = batch
            .into_iter()
            .map(|item| ((item.address, item.out.len(), None), item.out))
            .unzip();

        let _ = mem.read_raw_list_blocking(&mut read_list, priority, 0);

        let mut count = 0;
        for (out, (_, _, data)) in out_list.into_iter().zip(read_list) {
            if let Some(bytes) = data {
                out.copy_from_slice(&bytes);
                count += 1;
            }
        }

        count
    }

    fn write_raw(&self, addr: u64, data: &[u8], priority: i32) -> anyhow::Result<()> {
        let mem = MemAccess::from_handle(self.handle);
        mem.write_raw_blocking(addr, data, priority, 0)?;
        Ok(())
    }

    fn dump_memory(&self) -> anyhow::Result<Vec<u8>> {
        let mem = MemAccess::from_handle(self.handle);
        mem.dump_blocking().map(Into::into)
    }

    fn find_signature(
        &self,
        signature: &str,
        range_start: u64,
        range_end: u64,
    ) -> anyhow::Result<ffi::FindSignatureResult> {
        let mem = MemAccess::from_handle(self.handle);
        let ret = mem.find_sig_blocking(signature, range_start, range_end)?;
        match ret {
            None => Ok(ffi::FindSignatureResult {
                found: false,
                address: 0,
                data: Vec::new(),
            }),
            Some(r) => Ok(ffi::FindSignatureResult {
                found: true,
                address: r.address,
                data: r.data.into(),
            }),
        }
    }
}
