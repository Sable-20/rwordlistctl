use clap::{value_parser, ArgAction, Args, ValueEnum};

use crate::config;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Groups {
    Usernames,
    Passwords,
    Discovery,
    Fuzzing,
    Misc,
}

#[derive(Args, Debug)]
pub struct FetchArgs {
    #[arg(
        short = 'd',
        long = "decompress",
        help = "Decompress and remove the archive file", 
        action = ArgAction::SetTrue,
        default_value_t = false,
    )]
    pub decompress: bool,

    #[arg(
        short = 'w',
        long = "workers", 
        help = "Number of download workers",
        value_name = "COUNT",
        num_args = 1,
        require_equals = true,
        allow_negative_numbers = false,
        value_parser = value_parser!(u8).range(1..=100),
        default_value_t = config::get_worker_count(),
    )]
    pub workers: u8,

    #[arg(
        short = 'u',
        long = "user-agent",
        value_name = "USER_AGENT",
        help = "User agent to use for fetching",
        num_args = 1,
        require_equals = true,
        default_value = "rwordlistctl/0.1.0"
    )]
    pub user_agent: Option<String>,

    #[arg(
        short = 'b',
        long = "base-dir",
        value_name = "BASE_DIR",
        help = "Base directory to store wordlists",
        num_args = 1,
        require_equals = true,
        default_value = "/usr/share/wordlists"
    )]
    pub base_dir: Option<String>,

    #[arg(
        short = 'l',
        long = "wordlist",
        value_name = "WORDLISTS",
        help = "Wordlist to fetch",
        num_args(1..),
        require_equals = true,
        value_delimiter = ',',
        required = true,
    )]
    pub wordlists: Vec<String>,

    #[arg(
        short = 'g',
        long = "group",
        value_name = "GROUP",
        help = "Group of wordlists to fetch",
        num_args(1..=5),
        require_equals = true,
        value_delimiter = ',',
        value_enum
    )]
    pub group: Option<Vec<Groups>>,

    #[arg(
        short = 'r',
        long = "regex",
        help = "Use regex to find wordlists with your search term contained within the name",
        action = ArgAction::SetTrue,
        default_value_t = false,
    )]
    pub regex: bool,
}

#[derive(Args, Debug)]
pub struct SearchArgs {
    search_term: String,

    #[arg(
        short = 'l',
        long = "local",
        help = "Search for wordlists in the local archives",
        action = ArgAction::SetTrue,
        num_args = 0,
    )]
    pub local: bool,
    // #[arg(
    //     short = 'f',
    //     long = "fetch",
    //     help = "Fetch wordlists from the repository at the given indexes",
    //     action = ArgAction::Set,
    //     num_args(1..),
    //     require_equals = true,
    //     value_delimiter = ',',
    //     value_parser = value_parser!(u8).range(1..),
    // )]
    // pub fetch: Option<Vec<u8>>,
}

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(
        short = 'g',
        long = "group",
        value_name = "GROUP",
        help = "Group of wordlists to fetch",
        num_args(1..=5),
        require_equals = true,
        value_delimiter = ',',
        value_enum
    )]
    pub group: Option<Vec<Groups>>,
}
