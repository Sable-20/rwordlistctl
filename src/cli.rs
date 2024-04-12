use clap::{
    crate_authors, crate_description, crate_version, value_parser, Arg, ArgAction, Command,
};

pub fn build_cli() -> Command {
    Command::new("rwordlistctl")
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .long_about(None)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("fetch")
                .about("Fetch wordlists")
                .long_about(None)
                .args([
                    Arg::new("decompress")
                        .short('d')
                        .long("decompress")
                        .help("Decompress and remove the archive file")
                        .action(ArgAction::SetTrue)
                        .required(false),
                    Arg::new("workers")
                        .short('w')
                        .long("workers")
                        .help("Number of download workers")
                        .value_name("COUNT")
                        .num_args(1)
                        .require_equals(true)
                        .allow_negative_numbers(false)
                        .value_parser(value_parser!(u8).range(1..=100))
                        .default_value("10"),
                    Arg::new("user-agent")
                        .short('u')
                        .long("user-agent")
                        .value_name("USER_AGENT")
                        .help("User agent to use for fetching")
                        .num_args(1)
                        .require_equals(true)
                        .default_value("rwordlistctl/0.1.0"),
                    Arg::new("base-dir")
                        .short('b')
                        .long("base-dir")
                        .value_name("BASE_DIR")
                        .help("Base directory to store wordlists")
                        .num_args(1)
                        .require_equals(true)
                        .default_value("/usr/share/wordlists"),
                    Arg::new("wordlists")
                        .short('l')
                        .long("wordlist")
                        .value_name("WORDLISTS")
                        .help("Wordlist to fetch")
                        .num_args(1..)
                        .require_equals(true)
                        .value_delimiter(',')
                        ////.conflicts_with("group")
                        .required(false),
                    Arg::new("group")
                        .short('g')
                        .long("group")
                        .value_name("GROUP")
                        .help("Group to fetch wordlists")
                        .num_args(1)
                        .require_equals(true)
                        // //.conflicts_with("wordlists")
                        .required(false),
                    Arg::new("regex")
                        .short('r')
                        .long("regex")
                        .help("Use regex to search for wordlists")
                        .action(ArgAction::SetTrue),
                ]),
                Command::new("search")
                    .about("Search for wordlists")
                    .long_about(None)
                    .args([
                        Arg::new("regex")
                            .short('r')
                            .long("regex")
                            .help("Use regex to search for wordlists")
                            .action(ArgAction::SetTrue),
                        Arg::new("wordlist")
                            .long("wordlist")
                            .value_name("NAME")
                            .help("Name of the wordlist to search for")
                            .num_args(1..)
                            .require_equals(true)
                            .value_delimiter(',')
                            .required(false),
                        Arg::new("group")
                            .short('g')
                            .long("group")
                            .value_name("GROUP")
                            .help("Group of wordlists to search for")
                            .num_args(1)
                            .require_equals(true)
                            //.conflicts_with("name")
                            .required(false),

                ]),
                Command::new("list")
                    .about("List wordlists")
                    .long_about(None)
                    .args([
                        Arg::new("group")
                            .short('g')
                            .long("group")
                            .value_name("GROUP")
                            .help("Group of wordlists to list")
                            .num_args(1)
                            .require_equals(true)
                            .required(false),
                        Arg::new("number")
                            .short('n')
                            .long("number")
                            .value_name("COUNT")
                            .help("Number of wordlists to list")
                            .num_args(1)
                            .require_equals(true)
                            .value_parser(value_parser!(u8).range(1..=100))
                            .default_value("10"),
                        Arg::new("fetch")
                            .short('f')
                            .long("fetch")
                            .help("Fetch wordlists from the repository at the given indexes")
                            .action(ArgAction::SetTrue),
                    ]),

        ])
}
