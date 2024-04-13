complete -c rwordlistctl -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c rwordlistctl -n "__fish_use_subcommand" -s V -l version -d 'Print version'
complete -c rwordlistctl -n "__fish_use_subcommand" -f -a "fetch" -d 'Fetch wordlists'
complete -c rwordlistctl -n "__fish_use_subcommand" -f -a "search" -d 'Search for wordlists'
complete -c rwordlistctl -n "__fish_use_subcommand" -f -a "list" -d 'List wordlists'
complete -c rwordlistctl -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c rwordlistctl -n "__fish_seen_subcommand_from fetch" -s w -l workers -d 'Number of download workers' -r
complete -c rwordlistctl -n "__fish_seen_subcommand_from fetch" -s u -l user-agent -d 'User agent to use for fetching' -r
complete -c rwordlistctl -n "__fish_seen_subcommand_from fetch" -s b -l base-dir -d 'Base directory to store wordlists' -r
complete -c rwordlistctl -n "__fish_seen_subcommand_from fetch" -s l -l wordlist -d 'Wordlist to fetch' -r
complete -c rwordlistctl -n "__fish_seen_subcommand_from fetch" -s g -l group -d 'Group to fetch wordlists' -r
complete -c rwordlistctl -n "__fish_seen_subcommand_from fetch" -s d -l decompress -d 'Decompress and remove the archive file'
complete -c rwordlistctl -n "__fish_seen_subcommand_from fetch" -s r -l regex -d 'Use regex to search for wordlists'
complete -c rwordlistctl -n "__fish_seen_subcommand_from fetch" -s h -l help -d 'Print help'
complete -c rwordlistctl -n "__fish_seen_subcommand_from search" -l wordlist -d 'Name of the wordlist to search for' -r
complete -c rwordlistctl -n "__fish_seen_subcommand_from search" -s g -l group -d 'Group of wordlists to search for' -r
complete -c rwordlistctl -n "__fish_seen_subcommand_from search" -s r -l regex -d 'Use regex to search for wordlists'
complete -c rwordlistctl -n "__fish_seen_subcommand_from search" -s h -l help -d 'Print help'
complete -c rwordlistctl -n "__fish_seen_subcommand_from list" -s g -l group -d 'Group of wordlists to list' -r
complete -c rwordlistctl -n "__fish_seen_subcommand_from list" -s n -l number -d 'Number of wordlists to list' -r
complete -c rwordlistctl -n "__fish_seen_subcommand_from list" -s f -l fetch -d 'Fetch wordlists from the repository at the given indexes'
complete -c rwordlistctl -n "__fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c rwordlistctl -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from fetch; and not __fish_seen_subcommand_from search; and not __fish_seen_subcommand_from list; and not __fish_seen_subcommand_from help" -f -a "fetch" -d 'Fetch wordlists'
complete -c rwordlistctl -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from fetch; and not __fish_seen_subcommand_from search; and not __fish_seen_subcommand_from list; and not __fish_seen_subcommand_from help" -f -a "search" -d 'Search for wordlists'
complete -c rwordlistctl -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from fetch; and not __fish_seen_subcommand_from search; and not __fish_seen_subcommand_from list; and not __fish_seen_subcommand_from help" -f -a "list" -d 'List wordlists'
complete -c rwordlistctl -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from fetch; and not __fish_seen_subcommand_from search; and not __fish_seen_subcommand_from list; and not __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
