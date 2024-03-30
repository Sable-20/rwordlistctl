use std::{io::Write, path::PathBuf};

use ansi_term::Color::Red;

use clap::Parser;
use color_eyre::{eyre::eyre, Result, eyre::ensure};

use pretty_env_logger::env_logger;

use log::{debug, error, info, trace};

use crate::repo::{get_wordlist_by_name, get_wordlist_by_name_regex};

mod args;
mod config;
mod commands;
mod data;
mod repo;
mod units;

#[derive(Parser, Debug)]
#[command(
    version, 
    about, 
    long_about = None, 
    author,
    arg_required_else_help = true,
    subcommand_required = true
)]
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
    if std::path::Path::new("rwordlistctl.log").try_exists()? == false {
        std::fs::File::create("rwordlistctl.log")?;
    }
    let log_file = Box::new(std::fs::OpenOptions::new().append(true).open("rwordlistctl.log")?);
    

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

    match &cli.command {
        Some(commands::Command::Fetch(args)) => {
            trace!("Fetch called!");
            info!("{:#?}", args);
            info!(
                "User agent: {}",
                Red.bold().italic().paint(args.user_agent.as_ref().unwrap())
            );
            match (args.regex, args.decompress) {
                (true, true) => {
                    trace!("Decompress and regex");
                    for list in args.wordlists.iter() {
                        for list in get_wordlist_by_name_regex(list)? {
                            println!("{:?}", list.get_url());
                            // 
                            // retrieve_file(list.get_url(), 
                            //               args.decompress,
                            //               args.base_dir.as_ref().unwrap(), 
                            //               args.user_agent.as_ref().unwrap()
                            //               Data::new(list.get_size(), args.workers, get_unit(list.get_unit())).chunk_data()
                            // )?;

                            // retrieve_file function signature
                            // fn retrieve_file(url: &str, decompress: bool, base_dir: &str, user_agent: &str, data: Data) -> Result<()>    
                        }
                    }
                    error!("Implement decompression");
                    return Err(eyre!("Decompression not implemented"));
                }
                (true, false) => {
                    trace!("Regex only");
                    for list in args.wordlists.iter() {
                        for list in get_wordlist_by_name_regex(list)? {
                            println!("{:?}", list.get_url());
                        }
                    }
                }
                (false, true) => {
                    for list in args.wordlists.iter() {
                        if get_wordlist_by_name(list).is_err() {
                            return Err(eyre!("Failed to fetch wordlist"));
                        } else {
                            println!("{:?}", get_wordlist_by_name(list)?.get_url());
                        }
                    }
                }
                (false, false) => {
                    todo!("Normal fetch");
                }
            }
        },
        Some(commands::Command::Search(args)) => {
            debug!("Search args: {:#?}", args);
        },
        Some(commands::Command::List(args)) => {
            debug!("List args: {:#?}", args);
        },
        _ => unimplemented!(),
    }

    ensure!(cli.command.is_some(), eyre!("No command provided"));

    Ok(())
}
