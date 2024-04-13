
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'rwordlistctl' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'rwordlistctl'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'rwordlistctl' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('fetch', 'fetch', [CompletionResultType]::ParameterValue, 'Fetch wordlists')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search for wordlists')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List wordlists')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'rwordlistctl;fetch' {
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Number of download workers')
            [CompletionResult]::new('--workers', 'workers', [CompletionResultType]::ParameterName, 'Number of download workers')
            [CompletionResult]::new('-u', 'u', [CompletionResultType]::ParameterName, 'User agent to use for fetching')
            [CompletionResult]::new('--user-agent', 'user-agent', [CompletionResultType]::ParameterName, 'User agent to use for fetching')
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'Base directory to store wordlists')
            [CompletionResult]::new('--base-dir', 'base-dir', [CompletionResultType]::ParameterName, 'Base directory to store wordlists')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Wordlist to fetch')
            [CompletionResult]::new('--wordlist', 'wordlist', [CompletionResultType]::ParameterName, 'Wordlist to fetch')
            [CompletionResult]::new('-g', 'g', [CompletionResultType]::ParameterName, 'Group to fetch wordlists')
            [CompletionResult]::new('--group', 'group', [CompletionResultType]::ParameterName, 'Group to fetch wordlists')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Decompress and remove the archive file')
            [CompletionResult]::new('--decompress', 'decompress', [CompletionResultType]::ParameterName, 'Decompress and remove the archive file')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Use regex to search for wordlists')
            [CompletionResult]::new('--regex', 'regex', [CompletionResultType]::ParameterName, 'Use regex to search for wordlists')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'rwordlistctl;search' {
            [CompletionResult]::new('--wordlist', 'wordlist', [CompletionResultType]::ParameterName, 'Name of the wordlist to search for')
            [CompletionResult]::new('-g', 'g', [CompletionResultType]::ParameterName, 'Group of wordlists to search for')
            [CompletionResult]::new('--group', 'group', [CompletionResultType]::ParameterName, 'Group of wordlists to search for')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Use regex to search for wordlists')
            [CompletionResult]::new('--regex', 'regex', [CompletionResultType]::ParameterName, 'Use regex to search for wordlists')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'rwordlistctl;list' {
            [CompletionResult]::new('-g', 'g', [CompletionResultType]::ParameterName, 'Group of wordlists to list')
            [CompletionResult]::new('--group', 'group', [CompletionResultType]::ParameterName, 'Group of wordlists to list')
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'Number of wordlists to list')
            [CompletionResult]::new('--number', 'number', [CompletionResultType]::ParameterName, 'Number of wordlists to list')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Fetch wordlists from the repository at the given indexes')
            [CompletionResult]::new('--fetch', 'fetch', [CompletionResultType]::ParameterName, 'Fetch wordlists from the repository at the given indexes')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'rwordlistctl;help' {
            [CompletionResult]::new('fetch', 'fetch', [CompletionResultType]::ParameterValue, 'Fetch wordlists')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'Search for wordlists')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List wordlists')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'rwordlistctl;help;fetch' {
            break
        }
        'rwordlistctl;help;search' {
            break
        }
        'rwordlistctl;help;list' {
            break
        }
        'rwordlistctl;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
