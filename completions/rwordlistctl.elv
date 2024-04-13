
use builtin;
use str;

set edit:completion:arg-completer[rwordlistctl] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'rwordlistctl'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'rwordlistctl'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
            cand fetch 'Fetch wordlists'
            cand search 'Search for wordlists'
            cand list 'List wordlists'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'rwordlistctl;fetch'= {
            cand -w 'Number of download workers'
            cand --workers 'Number of download workers'
            cand -u 'User agent to use for fetching'
            cand --user-agent 'User agent to use for fetching'
            cand -b 'Base directory to store wordlists'
            cand --base-dir 'Base directory to store wordlists'
            cand -l 'Wordlist to fetch'
            cand --wordlist 'Wordlist to fetch'
            cand -g 'Group to fetch wordlists'
            cand --group 'Group to fetch wordlists'
            cand -d 'Decompress and remove the archive file'
            cand --decompress 'Decompress and remove the archive file'
            cand -r 'Use regex to search for wordlists'
            cand --regex 'Use regex to search for wordlists'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'rwordlistctl;search'= {
            cand --wordlist 'Name of the wordlist to search for'
            cand -g 'Group of wordlists to search for'
            cand --group 'Group of wordlists to search for'
            cand -r 'Use regex to search for wordlists'
            cand --regex 'Use regex to search for wordlists'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'rwordlistctl;list'= {
            cand -g 'Group of wordlists to list'
            cand --group 'Group of wordlists to list'
            cand -n 'Number of wordlists to list'
            cand --number 'Number of wordlists to list'
            cand -f 'Fetch wordlists from the repository at the given indexes'
            cand --fetch 'Fetch wordlists from the repository at the given indexes'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'rwordlistctl;help'= {
            cand fetch 'Fetch wordlists'
            cand search 'Search for wordlists'
            cand list 'List wordlists'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'rwordlistctl;help;fetch'= {
        }
        &'rwordlistctl;help;search'= {
        }
        &'rwordlistctl;help;list'= {
        }
        &'rwordlistctl;help;help'= {
        }
    ]
    $completions[$command]
}
