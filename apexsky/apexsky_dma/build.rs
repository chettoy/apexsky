mod cli {
    include!("src/cli.rs");
}

use std::{io, path::PathBuf};

use clap::CommandFactory;
use clap_complete::{
    aot::{Bash, Elvish, Fish, Zsh},
    generate_to, Generator,
};
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let outdir = std::env::var("OUT_DIR")?;

    let mut cmd = Cli::command();

    fn gen<G>(generator: G, cmd: &mut clap::Command, outdir: &str) -> Result<PathBuf, io::Error>
    where
        G: Generator,
    {
        generate_to(generator, cmd, "apexsky_dma", outdir)
    }

    // Generate completions for Bash, Elvish, Fish and Zsh.
    println!(
        "cargo:warning=bash completion file is generated: {:?}",
        gen(Bash, &mut cmd, &outdir)?
    );
    println!(
        "cargo:warning=elvish completion file is generated: {:?}",
        gen(Elvish, &mut cmd, &outdir)?
    );
    println!(
        "cargo:warning=fish completion file is generated: {:?}",
        gen(Fish, &mut cmd, &outdir)?
    );
    println!(
        "cargo:warning=zsh completion file is generated: {:?}",
        gen(Zsh, &mut cmd, &outdir)?
    );

    Ok(())
}
