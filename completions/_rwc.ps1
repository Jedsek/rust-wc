
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'rwc' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'rwc'
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
        'rwc' {
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'Print the byte counts')
            [CompletionResult]::new('--bytes', 'bytes', [CompletionResultType]::ParameterName, 'Print the byte counts')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Print the character counts')
            [CompletionResult]::new('--chars', 'chars', [CompletionResultType]::ParameterName, 'Print the character counts')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Print the word counts')
            [CompletionResult]::new('--words', 'words', [CompletionResultType]::ParameterName, 'Print the word counts')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Print the line counts')
            [CompletionResult]::new('--lines', 'lines', [CompletionResultType]::ParameterName, 'Print the line counts')
            [CompletionResult]::new('-L', 'L', [CompletionResultType]::ParameterName, 'Print the maximum line width (Bytes)')
            [CompletionResult]::new('--longest-line', 'longest-line', [CompletionResultType]::ParameterName, 'Print the maximum line width (Bytes)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('all', 'all', [CompletionResultType]::ParameterValue, 'Enabled all available options')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'rwc;all' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            break
        }
        'rwc;help' {
            [CompletionResult]::new('all', 'all', [CompletionResultType]::ParameterValue, 'Enabled all available options')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'rwc;help;all' {
            break
        }
        'rwc;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
