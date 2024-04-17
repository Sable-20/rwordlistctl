use std::io::Write;

use ansi_term::Color::{Green, Red};

use color_eyre::{
    eyre::eyre, Result
};

use pretty_env_logger::env_logger;

use log::{error, info};

use crate::repo::{get_wordlist_by_name, get_wordlist_by_name_regex};

mod args;
mod cli;
mod commands;
mod config;
mod fetch;
mod repo;
mod units;

// TODO: change default base dir based on whether the user is root or not

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
            .filter_level(log::LevelFilter::Info)
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

    let cli = cli::build_cli().get_matches();

    match cli.subcommand() {
        Some(("fetch", sub_matches)) => {
            let regex = sub_matches.get_one::<bool>("regex").unwrap();
            let decompress = sub_matches.get_one::<bool>("decompress").unwrap();
            let user_agent = sub_matches.get_one::<String>("user-agent").unwrap();
            let base_dir = sub_matches.get_one::<String>("base-dir").unwrap();
            let workers = sub_matches.get_one::<u8>("workers").unwrap();
            let wordlists = sub_matches
                                                            .get_many::<String>("wordlists")
                                                            .unwrap()
                                                            .map(|v| v.as_str())
                                                            .collect::<Vec<_>>();
            match (regex, decompress) {
                (true, true) => {
                    let mut size: f64 = 0.0;
                    let mut wordlists_to_fetch = vec![];
                    for list in wordlists.iter() {
                        for list in get_wordlist_by_name_regex(list)? {
                            size += list.get_size();
                            wordlists_to_fetch.push(list);
                        }
                    }
                    if size >= 1_000_000_000.0 {
                        let ans = inquire::Confirm::new("Are you sure you want to download this many wordlists? The size is >= 1.0 GB")
                            .with_default(true)
                            .with_help_message("This is quite a lot of data")
                            .prompt()?;

                        match ans {
                            true => {
                                for list in wordlists_to_fetch.iter() {
                                    fetch::retrieve_file(
                                        list,
                                        *decompress,
                                        base_dir,
                                        user_agent,
                                        *workers as usize,
                                    )
                                    .await?;
                                }
                            }
                            false => {
                                info!("Aborting download");
                            }
                        }
                    } else {
                        for list in wordlists_to_fetch.iter() {
                            fetch::retrieve_file(
                                list,
                                *decompress,
                                base_dir,
                                user_agent,
                                *workers as usize,
                            )
                            .await?;
                        }
                    }
                    return Ok(());
                }
                (true, false) => {
                    let mut size: f64 = 0.0;
                    let mut wordlists_to_fetch = vec![];
                    for list in wordlists.iter() {
                        for list in get_wordlist_by_name_regex(list)? {
                            size += list.get_size();
                            wordlists_to_fetch.push(list);
                        }
                    }
                    if size >= 1_000_000_000.0 {
                        let ans = inquire::Confirm::new("Are you sure you want to download this many wordlists? The size is >= 1.0 GB")
                            .with_default(true)
                            .with_help_message("This is quite a lot of data")
                            .prompt()?;

                        match ans {
                            true => {
                                for list in wordlists_to_fetch.iter() {
                                    fetch::retrieve_file(
                                        list,
                                        *decompress,
                                        base_dir,
                                        user_agent,
                                        *workers as usize,
                                    )
                                    .await?;
                                }
                            }
                            false => {
                                info!("Aborting download");
                            }
                        }
                    } else {
                        for list in wordlists_to_fetch.iter() {
                            fetch::retrieve_file(
                                list,
                                *decompress,
                                base_dir,
                                user_agent,
                                *workers as usize,
                            )
                            .await?;
                        }
                    }
                    return Ok(());
                }
                (false, true) => {
                    let mut size: f64 = 0.0;
                    let mut wordlists_to_fetch = vec![];
                    for list in wordlists.iter() {
                        let lst = get_wordlist_by_name(list)?;
                        size += lst.get_size();
                        wordlists_to_fetch.push(lst);
                    }
                    if size >= 1_000_000_000.0 {
                        let ans = inquire::Confirm::new("Are you sure you want to download this many wordlists? The size is >= 1.0 GB")
                            .with_default(true)
                            .with_help_message("This is quite a lot of data")
                            .prompt()?;

                        match ans {
                            true => {
                                std::thread::scope(|s| {
                                    for list in wordlists_to_fetch.iter() {
                                        s.spawn(|| async {
                                            let _ = fetch::retrieve_file(
                                                list,
                                                *decompress,
                                                base_dir,
                                                user_agent,
                                                *workers as usize,
                                            ).await;
                                        });
                                    }
                                });
                            }
                            false => {
                                info!("Aborting download");
                            }
                        }
                    } else {
                        for list in wordlists_to_fetch.iter() {
                            fetch::retrieve_file(
                                list,
                                *decompress,
                                base_dir,
                                user_agent,
                                *workers as usize,
                            )
                            .await?;
                        }
                    }
                    return Ok(());
                }
                (false, false) => {
                    let mut size: f64 = 0.0;
                    let mut wordlists_to_fetch = vec![];
                    for list in wordlists.iter() {
                        let lst = get_wordlist_by_name(list)?;
                        size += lst.get_size();
                        wordlists_to_fetch.push(lst);
                    }
                    if size >= 1_000_000_000.0 {
                        let ans = inquire::Confirm::new("Are you sure you want to download this many wordlists? The size is >= 1.0 GB")
                            .with_default(true)
                            .with_help_message("This is quite a lot of data")
                            .prompt()?;

                        match ans {
                            true => {
                                for list in wordlists_to_fetch.iter() {
                                    fetch::retrieve_file(
                                        list,
                                        *decompress,
                                        base_dir,
                                        user_agent,
                                        *workers as usize,
                                    )
                                    .await?;
                                }
                            }
                            false => {
                                info!("Aborting download");
                            }
                        }
                    } else {
                        for list in wordlists_to_fetch.iter() {
                            fetch::retrieve_file(
                                list,
                                *decompress,
                                base_dir,
                                user_agent,
                                *workers as usize,
                            )
                            .await?;
                        }
                    }
                    return Ok(());
                }
            }
        }
        Some(("search", sub_matches)) => {
            if sub_matches.get_one::<Vec<String>>("wordlist").is_none()
                && sub_matches.get_one::<Vec<args::Groups>>("group").is_none()
            {
                return Err(eyre!("No search term provided"));
            }
            let wordlists = sub_matches.get_many::<String>("wordlists").unwrap().map(|v| v.as_str()).collect::<Vec<&str>>();
            let group = sub_matches.get_many::<args::Groups>("group").unwrap().map(|v| v.as_ref()).collect::<Vec<&args::Groups>>();
            let local = sub_matches.get_flag("local");
            if !wordlists.is_empty() {
                for list in wordlists.iter() {
                    let wordlist = get_wordlist_by_name(list)?;
                    println!("{:#?}", wordlist);
                }
            }
            if !group.is_empty() {
                for group in group.iter() {
                    let wordlists = repo::get_wordlist_by_group(*group)?;
                    for wordlist in wordlists {
                        println!("{:#?}", wordlist);
                    }
                }
            }
            if local {
                for list in wordlists.iter() {
                    let wordlist = get_wordlist_by_name(list)?;
                    let path = format!("/usr/share/wordlists/{}", wordlist.get_name());
                    if std::path::Path::new(&path).try_exists()? {
                        // pretty display info
                        println!("Wordlist found at: {}", path);
                    } else {
                        error!("Wordlist not found at: {}", path);
                    }
                }
            }
        }
        Some(("list", sub_matches)) => {
            let mut lists = vec![];
            let mut ctr = 0;
            let count = sub_matches.get_one::<u8>("number").unwrap();
            let fetch = sub_matches.get_flag("fetch");
            let groups = sub_matches.get_many::<String>("group").unwrap().map(|v| v.as_str()).collect::<Vec<&str>>();
            // let groups = sub_matches.get_many::<args::Groups>("group").unwrap().map(|v| v.as_ref()).collect::<Vec<&args::Groups>>();
            for group in groups.iter() {
                for lst in repo::get_wordlist_by_group(&crate::args::Groups::from(*group))? {
                    lists.push(lst);
                    if ctr == *count {
                        break;
                    }
                    ctr += 1;
                }
            }
            if !fetch {
                println!("{} Possible lists", Green.bold().underline().paint(lists.len().to_string()));
                for i in 0..lists.len() {
                    println!("{}. {} - {} {}", 
                        ansi_term::Colour::Cyan.bold().paint(i.to_string()), 
                        Green.italic().paint(lists[i].get_name()),
                        Red.bold().paint(units::readable_size(lists[i].get_size().ceil() as usize).0.to_string()),
                        Red.bold().paint(units::readable_size(lists[i].get_size().ceil() as usize).1.to_string())
                    );
                }
                // println!("Possible lists: {:#?}", &lists.iter().map(|list| list.get_name()).collect::<Vec<&str>>());
                return Ok(());
            }
            let names = lists
                .iter()
                .map(|list| list.get_name())
                .collect::<Vec<&str>>();
            let sizes = lists
                .iter()
                .map(|list| units::readable_size(list.get_size().ceil() as usize))
                .collect::<Vec<(f64, units::Units)>>();
            let mut sizes_with_text = vec![];
            for (size, unit) in sizes.iter() {
                sizes_with_text.push(format!(" {:.2} {}", size, unit));
            }
            let confirmation_opts: Vec<_> = names.iter().zip(sizes_with_text.iter()).collect();
            let opts = lists
                .iter()
                .map(|list| list.get_name().to_owned())
                .collect::<Vec<String>>();
            let confirmation = inquire::Confirm::new(
                &format!(
                    "Are you sure you want to download these files? \n
                    {}",
                    confirmation_opts
                        .iter()
                        .map(|(name, size)| {
                            format!(
                                "{} - [{}]", 
                                name, 
                                size.to_string())
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            )
            .with_default(true)
            .with_help_message("These are all the lists you selected")
            .prompt()?;
            match confirmation {
                true => {
                    for list in lists.iter() {
                        crate::fetch::retrieve_file(
                            list,
                            true,
                            "/usr/share/wordlists",
                            "rwordlistctl/0.1.0",
                            config::get_worker_count() as usize,
                        )
                        .await?;
                    }
                }
                false => {
                    // let formatter: inquire::formatter::MultiOptionFormatter<'_, &str> = &|opt| format!("{} different lists", opt.len());
                    let multiselect =
                        inquire::MultiSelect::new("Select the lists you want to download", opts)
                            //.with_formatter(formatter)
                            .prompt()?;
                    for name in multiselect {
                        let list = get_wordlist_by_name(&name)?;
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
        }
        _ => {
            return Err(eyre!("No command provided"));
        }
    }
    info!("Time elapsed: {:.2?} seconds", now.elapsed().as_secs_f64());
    Ok(())
}
