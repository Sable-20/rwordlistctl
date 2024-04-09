use std::io::Write;

use ansi_term::Color::Red;

use clap::Parser;
use color_eyre::{eyre::eyre, Result};

use pretty_env_logger::env_logger;

use log::{debug, error, info, trace};

use crate::repo::{get_wordlist_by_name, get_wordlist_by_name_regex};

mod args;
mod commands;
mod config;
mod data;
mod fetch;
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
    // #[arg(
    //     short = 'c',
    //     long = "config",
    //     value_name = "CONFIG",
    //     help = "Path to the configuration file",
    //     default_value = "/usr/share/rwordlistctl/.config/config.toml",
    // )]
    // config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<commands::Command>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let now = std::time::Instant::now();

    if std::path::Path::new("rwordlistctl.log").try_exists()? == false {
        std::fs::File::create("rwordlistctl.log")?;
    }
    let log_file = Box::new(
        std::fs::OpenOptions::new()
            .append(true)
            .open("rwordlistctl.log")?,
    );

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
        #[cfg(debug_assertions)]
        _ => pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Trace)
            .target(env_logger::Target::Stdout)
            .init(),
        #[cfg(not(debug_assertions))]
        _ => pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Warn)
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
                            // println!("Fetching: {}", list.get_name());
                            fetch::retrieve_file(
                                &list,
                                args.decompress,
                                args.base_dir.as_ref().unwrap(),
                                args.user_agent.as_ref().unwrap(),
                                args.workers as usize,
                            )
                            .await?;
                        }
                    }
                }
                (true, false) => {
                    trace!("Regex only");
                    for list in args.wordlists.iter() {
                        for list in get_wordlist_by_name_regex(list)? {
                            fetch::retrieve_file(
                                &list,
                                args.decompress,
                                args.base_dir.as_ref().unwrap(),
                                args.user_agent.as_ref().unwrap(),
                                args.workers as usize,
                            )
                            .await?;
                        }
                    }
                }
                (false, true) => {
                    for list in args.wordlists.iter() {
                        if let Ok(list) = get_wordlist_by_name(list) {
                           fetch::retrieve_file(
                                &list, 
                                args.decompress, 
                                args.base_dir.as_ref().unwrap(), 
                                args.user_agent.as_ref().unwrap(),
                                args.workers as usize
                            )
                            .await?;
                        } else {
                            error!("Failed to fetch wordlist");
                        }
                    }
                }
                (false, false) => {
                    for list in args.wordlists.iter() {
                        fetch::retrieve_file(
                            &get_wordlist_by_name(&list)?,
                            args.decompress,
                            args.base_dir.as_ref().unwrap(),
                            args.user_agent.as_ref().unwrap(),
                            args.workers as usize,
                        )
                        .await?;
                    }
                    info!("File fetched successfully");
                }
            }
        }
        Some(commands::Command::Search(args)) => {
            debug!("Search args: {:#?}", args);
        }
        Some(commands::Command::List(args)) => {
            debug!("List args: {:#?}", args);
            if !args.fetch {
                for groups in args.group.iter() {
                    for group in groups {
                        let lists = repo::get_wordlist_by_group(*group)?;
                        let names = lists
                            .iter()
                            .map(|list| list.get_name().to_owned())
                            .collect::<Vec<String>>();
                        println!("Possible lists: {:#?}\n\nWith {} more options", &names[0..args.number as usize], &names.len() - args.number as usize);
                        info!("Size of lists: {:?}", &lists.iter().map(|list| list.get_size()).collect::<Vec<f64>>()[0..args.number as usize]);
                    }
                }
                return Ok(());
            }
            for groups in args.group.iter() {
                for group in groups {
                    let lists = repo::get_wordlist_by_group(*group)?;
                    for list in lists {
                        // use config to get the base dir and user agent
                        crate::fetch::retrieve_file(
                            &list,
                            true,
                            "/usr/share/wordlists",
                            "rwordlistctl/0.1.0",
                            config::get_worker_count() as usize,
                        )
                        .await?;
                    }
                }
            }
            return Ok(());
        }
        &None => {
            return Err(eyre!("No command provided"));
        }
    }

    info!("Time elapsed: {:.2?} seconds", now.elapsed().as_secs_f64());
    Ok(())
}
