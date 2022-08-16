
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
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-b', 'b', [CompletionResultType]::ParameterName, 'Print the count of bytes')
            [CompletionResult]::new('--bytes', 'bytes', [CompletionResultType]::ParameterName, 'Print the count of bytes')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Print the count of chars')
            [CompletionResult]::new('--chars', 'chars', [CompletionResultType]::ParameterName, 'Print the count of chars')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Print the count of words')
            [CompletionResult]::new('--words', 'words', [CompletionResultType]::ParameterName, 'Print the count of words')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Print the count of lines')
            [CompletionResult]::new('--lines', 'lines', [CompletionResultType]::ParameterName, 'Print the count of lines')
            [CompletionResult]::new('-L', 'L', [CompletionResultType]::ParameterName, 'Print the length of the longest line')
            [CompletionResult]::new('--longest-line', 'longest-line', [CompletionResultType]::ParameterName, 'Print the length of the longest line')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
