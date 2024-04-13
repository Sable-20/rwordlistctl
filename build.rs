use clap::ValueEnum;
use clap_complete::Shell;
use std::env;
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    // let outdir = match env::var_os("OUT_DIR") {
    //     Some(outdir) => outdir,
    //     None => return Ok(()),
    // };

    let mut app = build_cli();
    for &shell in Shell::value_variants() {
        clap_complete::generate_to(shell, &mut app, "rwordlistctl", "completions")?;
    }

    let man = clap_mangen::Man::new(app);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write("man/rwordlistctl.1", buffer)?;

    Ok(())
}
