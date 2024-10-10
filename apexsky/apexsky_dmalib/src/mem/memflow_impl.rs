use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{anyhow, Context};
use core::time;
use memflow::prelude::v1::*;
use memflow_win32::prelude::v1::*;
use obfstr::obfstr as s;
use pe_parser::pe::parse_portable_executable;
use tracing::instrument;

use super::{MemProc, ProcessStatus};

pub struct MemflowOs {
    inventory: Inventory,
    os: OsInstanceArcBox<'static>,
}

pub struct MemflowProc<'a> {
    base_addr: Address,
    status: ProcessStatus,
    proc: ProcessInstanceArcBox<'a>,
}

const MZ_HEADER: u16 = 0x5A4D;

impl std::fmt::Debug for MemflowOs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(s!("MemflowOs"))
            .field(
                s!("inventory"),
                &(
                    self.inventory.available_connectors(),
                    self.inventory.available_os(),
                ),
            )
            .field(s!("os"), &self.os.info())
            .finish()
    }
}

impl<'a> std::fmt::Debug for MemflowProc<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        f.debug_struct(s!("MemflowProc"))
            .field(s!("base_addr"), &self.base_addr)
            .field(s!("status"), &self.status)
            .field(s!("proc"), &self.proc.info())
            .finish()
    }
}

impl MemflowOs {
    pub fn new(choose_connector: super::MemConnector) -> anyhow::Result<Self> {
        // load all available plugins
        let mut inventory = Inventory::scan();

        if cfg!(unix) {
            inventory
                .add_dir(PathBuf::from(s!("/usr/lib/memflow")))
                .ok();

            if let Some(original_home) = apexsky_utils::get_runner_home_dir() {
                inventory
                    .add_dir(original_home.join(".local").join("lib").join(s!("memflow")))
                    .ok();
            }
        }

        tracing::info!("{}", s!("inventory initialized"));

        let (connector_name, connector_args, os_name) = {
            match choose_connector {
                super::MemConnector::MemflowKvm => (
                    {
                        if Path::new(s!("/dev/memflow")).exists() {
                            s!("kvm").to_string()
                        } else {
                            s!("qemu").to_string()
                        }
                    },
                    String::new(),
                    s!("win32").to_string(),
                ),
                super::MemConnector::MemflowNative => {
                    (String::new(), String::new(), s!("native").to_string())
                }
                super::MemConnector::MemflowPCILeech => (
                    s!("pcileech").to_string(),
                    s!(":device=FPGA").to_string(),
                    s!("win32").to_string(),
                ),
                super::MemConnector::PCILeech(v) => {
                    anyhow::bail!(format!("{}{}{}", s!("no such connector `"), v, s!("`")))
                }
            }
        };

        let connector = if connector_name.is_empty() {
            None
        } else {
            tracing::info!("{}{}{}", s!("Using "), connector_name, s!(" connector."));

            let connector_args = if connector_args.is_empty() {
                None
            } else {
                connector_args
                    .parse()
                    .map(Some)
                    .context(s!("unable to parse connector arguments").to_string())?
            };

            inventory
                .create_connector(&connector_name, None, connector_args.as_ref())
                .map(Some)
                .context(format!(
                    "{}{}{}",
                    s!("unable to create "),
                    connector_name,
                    s!(" connector")
                ))?
        };

        let os = inventory
            .create_os(&os_name, connector, None)
            .context(format!(
                "{}{}{}{}{}",
                s!("unable to create "),
                os_name,
                s!("instance with "),
                connector_name,
                s!(" connector")
            ))?;

        Ok(MemflowOs { inventory, os })
    }
}

impl super::MemOs for MemflowOs {
    #[instrument]
    fn open_proc<'a>(&'a mut self, name: &str) -> anyhow::Result<super::MemProcImpl<'a>> {
        const LOG_MODULE_LIST: bool = false;

        let mut os = self.os.clone();
        let mut proc = os.process_by_name(name)?;

        let proc_info = proc.info();

        tracing::info!(
            ?name,
            ?proc_info,
            "{}{:x}{}",
            s!("Process Found: 0x"),
            proc_info.address,
            s!("] "),
        );

        if LOG_MODULE_LIST {
            for module_info in proc.module_list().inspect_err(|e| tracing::warn!(%e))? {
                tracing::warn!(
                    test = "module_list",
                    name = module_info.name.to_string(),
                    ?module_info,
                    "{}{:x}{}{:x}",
                    s!("module found: 0x"),
                    module_info.address,
                    s!("] 0x"),
                    module_info.base,
                );
            }
        }

        let module_base = proc
            .module_by_name(name)
            .inspect_err(|e| tracing::warn!(%e))
            .ok()
            .and_then(|module_info| {
                tracing::info!(
                    ?name,
                    ?module_info,
                    "{}{:x}{}{:x}",
                    s!("module found: 0x"),
                    module_info.address,
                    s!("] 0x"),
                    module_info.base,
                );
                Some(module_info.base)
            });

        if let Some(module_base) = module_base {
            Ok(super::MemProcImpl::Memflow(MemflowProc {
                base_addr: module_base,
                status: ProcessStatus::FoundReady,
                proc: self.os.process_by_name(name)?,
            }))
        } else {
            let connector = self
                .os
                .clone()
                .cast_impl_physicalmemory()
                .context(s!("unable to cast os connector").to_string())?;
            let mut win32_kernel = Box::new(
                Win32Kernel::builder(connector)
                    .build_default_caches()
                    .build()
                    .context(
                        s!("unable to create win32 instance with qemu connector").to_string(),
                    )?,
            );

            let section_base_address = win32_kernel.process_by_name(name)?.proc_info.section_base;

            let mut proc = self.os.process_by_name(name)?;

            // Credit: https://www.unknowncheats.me/forum/anti-cheat-bypass/635533-eac-dtb-fix-memflow-rust-paste-ready.html
            // the idea here is that since EAC sets CR3 to be invalid, memflow cannot resolve the correct DTB.
            // DTBs must be page aligned, meaning we can iterate across every usize value incrementing by
            // 4096 (0x1000) bytes, and we will quickly (~600ms) find the correct DTB.
            // we can verify it is correct by reading the MZ header with our generated DTB value.
            // once it is fixed, we will never have to touch it again, as we don't need to resolve the process
            // each time we perform a read or write with memflow!
            if proc.read::<u16>(section_base_address)? != MZ_HEADER {
                if let Some(dtb) = (0..=usize::MAX).step_by(0x1000).find(|&dtb| {
                    proc.set_dtb(Address::from(dtb), Address::invalid())
                        .unwrap();
                    if dtb > 0x1000 * 500 {
                        let progress = dtb / 0x1000;
                        if progress % 200000 == 0 {
                            tracing::info!(?progress);
                        }
                    }
                    if proc.read::<u16>(section_base_address).unwrap() != MZ_HEADER {
                        return false;
                    }
                    tracing::info!("{}{:X}", s!("testing dtb: "), dtb);
                    if let Ok(pe_dat) = proc.read_raw(section_base_address, 0x1000) {
                        // parsing the PE is unneeded here, but sometimes you can find two dtbs that yield the MZ header.
                        // if you are unable to read game addresses, add additional verification here,
                        // such as trying to read localplayer and seeing if it resolves.
                        if parse_portable_executable(pe_dat.as_slice()).is_ok() {
                            return true;
                        }
                    }
                    false
                }) {
                    tracing::info!("{}{:X}", s!("[+] dtb: "), dtb);
                    Ok(super::MemProcImpl::Memflow(MemflowProc {
                        base_addr: section_base_address,
                        status: ProcessStatus::FoundReady,
                        proc,
                    }))
                } else {
                    tracing::error!("{}", s!("[-] Failed to find module"));
                    anyhow::bail!("{}", s!("Failed to find dtb"));
                }
            } else {
                tracing::error!("{}", s!("f76f0a2e-80bb-4750-a295-0b065dc1c73b"));
                Ok(super::MemProcImpl::Memflow(MemflowProc {
                    base_addr: section_base_address,
                    status: ProcessStatus::FoundReady,
                    proc,
                }))
            }
        }
    }
}

impl<'a> MemProc for MemflowProc<'a> {
    #[inline]
    fn get_base_addr(&self) -> u64 {
        self.base_addr.to_umem()
    }

    fn set_base_addr(&mut self, base_addr: u64) {
        self.base_addr = base_addr.into();
    }

    #[instrument]
    fn check_proc_status(&mut self) -> ProcessStatus {
        if self.base_addr.is_null() {
            self.status = ProcessStatus::NotFound;
            return self.status;
        }

        if self.status == ProcessStatus::FoundReady {
            let mut c: u16 = 0;
            self.read_raw_into(self.base_addr.to_umem(), dataview::bytes_mut(&mut c))
                .ok();

            if c != MZ_HEADER {
                self.status = ProcessStatus::NotFound;
                self.base_addr = Address::null();
            }
        }

        self.status
    }

    #[instrument(skip_all)]
    fn speed_test(&mut self) {
        tracing::debug!("{}", s!("029194cf-ce9a-42aa-91c7-e35108e9ddb0"));
        if self.status != ProcessStatus::FoundReady {
            let err = anyhow!(s!("proc instance is None").to_string());
            tracing::error!(%err);
            return;
        }
        let proc = &mut self.proc;

        println!("{}", s!("Received metadata:"));
        let metadata = proc.metadata();
        tracing::debug!(?metadata);
        println!("{}{:x}", s!("max_address=0x"), metadata.max_address);
        println!("{}{:x}", s!("real_size=0x"), metadata.real_size);
        println!("{}{}", s!("readonly="), metadata.readonly);

        println!("{}", s!("== speed test start =="));

        let addr = self.base_addr;
        let start = Instant::now();
        for counter in 0..5000 {
            let mut buf = vec![0; 0x1000];
            proc.read_raw_into(addr, &mut buf)
                .expect(s!("speed_test: unable to read process memory"));

            if counter % 1000 == 0 {
                let elapsed = start.elapsed().as_millis() as f64;
                if elapsed > 0.0 {
                    let result = (
                        format!(
                            "{}{}",
                            (f64::from(counter)) / elapsed * 1000.0,
                            s!(" reads/sec")
                        ),
                        format!("{}{}", elapsed / (f64::from(counter)), s!(" ms/read")),
                    );
                    tracing::debug!(result0 = result.0, result1 = result.1);
                    println!("{}", result.0);
                    println!("{}", result.1);
                }
            }
        }

        println!("{}", s!("== speed test end =="));
    }

    #[instrument(skip_all)]
    fn read_raw_into(&mut self, addr: u64, out: &mut [u8]) -> anyhow::Result<()> {
        if self.status != ProcessStatus::FoundReady {
            anyhow::bail!(s!("proc instance is None").to_string());
        }

        let addr = Address::from(addr);

        let mut result = Ok(());
        for i in 0..2 {
            result = self.proc.read_raw_into(addr, out);
            match &result {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    tracing::debug!(%e, retry=i);
                    if i > 1 {
                        std::thread::sleep(time::Duration::from_millis(2));
                    }
                }
            }
        }
        // if let Err(e) = result {
        //     tracing::warn!(%e);
        // }
        //Ok(())
        Ok(result?)
    }

    #[instrument(skip_all)]
    fn write_raw(&mut self, addr: u64, data: &[u8]) -> anyhow::Result<()> {
        if self.status != ProcessStatus::FoundReady {
            anyhow::bail!(s!("proc instance is None").to_string());
        }

        let addr = Address::from(addr);

        let mut result = Ok(());
        for i in 0..3 {
            result = self.proc.write_raw(addr, data);
            match &result {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    tracing::debug!(%e, retry=i);
                    if i > 1 {
                        std::thread::sleep(time::Duration::from_millis(2));
                    }
                }
            }
        }
        Ok(result?)
        // if let Err(e) = result {
        //     tracing::warn!(%e);
        // }
        //Ok(())
    }
}
