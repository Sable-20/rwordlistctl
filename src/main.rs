use std::{io::Write, path::PathBuf};

use ansi_term::Color::Red;

use clap::Parser;
use color_eyre::{
    eyre::{ensure, eyre}, owo_colors::OwoColorize, Result
};

use pretty_env_logger::env_logger;

use log::{debug, error, info, trace, warn};

use crate::repo::*;

mod args;
mod commands;
mod data;
mod repo;
mod units;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, author)]
struct RWordlistctl {
    #[arg(
        short = 'c',
        long = "config",
        value_name = "CONFIG",
        help = "Path to the configuration file",
        default_value = "config/config.toml"
    )]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<commands::Command>,
}

fn main() -> Result<()> {
    let log_file = Box::new(std::fs::File::create("rwordlistctl.log")?);

    match std::env::var("RUST_LOG_STYLE") {
        Ok(s) if s == "SYSTEMD" => {
            pretty_env_logger::formatted_builder()
                .filter_level(log::LevelFilter::Trace)
                .format(|buf, record| {
                    writeln!(
                        buf,
                        "<{}>{}: {}",
                        match record.level() {
                            log::Level::Error => 3,
                            log::Level::Warn => 4,
                            log::Level::Info => 6,
                            log::Level::Debug => 7,
                            log::Level::Trace => 7,
                        },
                        record.target(),
                        record.args()
                    )
                })
                .target(env_logger::Target::Pipe(log_file))
                .init();
            info!("Using systemd syslog friendly logging");
        }
        _ => pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Trace)
            .target(env_logger::Target::Stdout) 
            .init(),
    }
    color_eyre::install()?;

    println!(
        "--==[ {project} by {organization} ]==--",
        project = Red.bold().paint("rwordlistctl"),
        organization = Red.bold().paint("Blackarch Linux")
    );

    #[allow(unused_variables)]
    let cli: RWordlistctl = RWordlistctl::parse();

    if let Some(config) = cli.config {
        info!("Using configuration file: {:?}", config);
    }

    if cli.command.is_none() {
        error!("No command provided");
        // return Err(eyre!("No command provided"));
        // std::process::exit(-1); 
    }

    match &cli.command {
        Some(commands::Command::Fetch(args)) => {
            trace!("Fetch called!");
            info!("{:#?}", args);
            info!("User agent: {}", Red.bold().italic().paint(args.user_agent.as_ref().unwrap()));
            match (args.regex, args.decompress) {
                (true, true) => {
                    todo!("Decompress and regex");
                },
                (true, false) => {
                    todo!("Regex");
                },
                (false, true) => {
                    todo!("Decompress");
                },
                (false, false) => {
                    todo!("Normal fetch");
                },
            }
            // if args.regex {
            //     info!("Using regex");
            //     for list in get_wordlist_by_name_regex("rockyou")?.iter() {
            //         println!("{}", list.get_url());
            //     }
            // }
            // println!("{}", get_wordlist_by_name("deepmagic")?.get_url());
        }
        _ => unimplemented!(),
    }

    // ensure!(cli.command.is_some(), eyre!("No command provided"));

    Ok(())
}
