use std::{str::FromStr, thread::sleep, time::Duration};

use memprocfs::{Vmm, VmmProcess};
use obfstr::obfstr as s;

#[allow(dead_code)]
#[derive(Debug)]
struct DtbInfo {
    index: u32,
    process_id: u32,
    dtb: u64,
    kernel_addr: u64,
    name: String,
}

impl FromStr for DtbInfo {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let index = parts.next().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, s!("Missing index"))
        })?;
        let process_id = parts.next().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, s!("Missing process_id"))
        })?;
        let dtb = parts.next().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, s!("Missing dtb"))
        })?;
        let kernel_addr = parts.next().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, s!("Missing kernel_addr"))
        })?;
        let name = parts.next().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, s!("Missing name"))
        })?;

        Ok(DtbInfo {
            index: parse_hex(index)?
                .try_into()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
            process_id: u32::from_str(process_id)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
            dtb: parse_hex(dtb)?,
            kernel_addr: parse_hex(kernel_addr)?,
            name: String::from_str(name)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?,
        })
    }
}

fn parse_hex(s: &str) -> Result<u64, std::io::Error> {
    u64::from_str_radix(s, 16).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

#[tracing::instrument(skip_all)]
pub fn fix_dtb<'a, 'b>(
    vmm: &Vmm<'a>,
    process: &VmmProcess<'b>,
    module_name: &str,
) -> anyhow::Result<bool> {
    if process.get_module_base(module_name).is_ok() {
        tracing::info!("{}", s!("skip fix_dtb"));
        return Ok(true);
    }

    // Wait for procinfo ready
    loop {
        let vfs_file_data = vmm.vfs_read(s!("/misc/procinfo/progress_percent.txt"), 4, 0)?;

        let progress_percent: i32 = String::from_utf8_lossy(&vfs_file_data).parse()?;
        tracing::info!("{}{}{}", s!("procinfo "), progress_percent, s!("%"));
        if progress_percent == 100 {
            break;
        } else {
            sleep(Duration::from_millis(200));
        }
    }

    // Read dtb.txt
    let dtb_txt = {
        let Some(dtb_txt_size) = vmm
            .vfs_list(s!("/misc/procinfo/"))?
            .iter()
            .find_map(|file| {
                if file.name == s!("dtb.txt") {
                    Some(file.size)
                } else {
                    None
                }
            })
        else {
            anyhow::bail!("{}", s!("Failed to stat dtb.txt"));
        };

        let vfs_file_data =
            vmm.vfs_read(s!("/misc/procinfo/dtb.txt"), dtb_txt_size.try_into()?, 0)?;
        String::from_utf8(vfs_file_data)?
    };
    tracing::trace!(?dtb_txt);

    let the_line = dtb_txt.lines().try_find(|&l| -> anyhow::Result<bool> {
        let info = DtbInfo::from_str(l)?;

        if info.process_id == 0 || info.name == module_name {
            tracing::info!(?info);
            vmm.set_config(
                memprocfs::CONFIG_OPT_PROCESS_DTB | process.pid as u64,
                info.dtb,
            )?;
            if let Ok(module_base) = process.get_module_base(module_name) {
                if module_base != 0 {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    })?;

    if let Some(line) = the_line {
        tracing::info!("{}{}", s!("[+] dtb: "), line);
        Ok(true)
    } else {
        tracing::warn!("{}", s!("[-] Failed to find module"));
        Ok(false)
    }
}
