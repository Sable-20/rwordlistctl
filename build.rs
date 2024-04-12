use clap::ValueEnum;
use clap_complete::Shell;
use std::env;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => return Ok(()),
    };

    let mut app = build_cli();
    for &shell in Shell::value_variants() {
        clap_complete::generate_to(shell, &mut app, "rwordlistctl", &outdir)?;
    }

    Ok(())
}
