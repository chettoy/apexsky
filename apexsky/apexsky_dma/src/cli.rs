use clap::{Parser, Subcommand};
use clap_num::maybe_hex;

#[derive(Parser, Debug, PartialEq)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Disable TUI menu
    #[arg(long)]
    pub debug: bool,

    /// Bypass time_date_stamp check
    #[arg(long)]
    pub force_bypass_check: bool,

    /// Set target process name
    #[arg(long)]
    pub proc_name: Option<String>,

    /// Specify the module base address
    #[arg(long, value_parser=maybe_hex::<u64>)]
    pub module_base: Option<u64>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Commands {
    /// Show menu only
    Menu,

    /// Starts with MemProcFS FPGA (default)
    Fpga,

    /// Starts with memflow KVM/QEMU connector
    Kvm,

    /// Starts with memflow native/linux connector
    #[clap(aliases = &["no-kvm", "nokvm", "nodma", "linux"])]
    Native,

    /// Starts with MemProcFS LeechCore
    #[clap(alias = "pcileech")]
    Leechcore { device: String },
}
