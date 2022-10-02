
use builtin;
use str;

set edit:completion:arg-completer[rwc] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'rwc'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'rwc'= {
            cand -b 'Print the byte counts'
            cand --bytes 'Print the byte counts'
            cand -c 'Print the character counts'
            cand --chars 'Print the character counts'
            cand -w 'Print the word counts'
            cand --words 'Print the word counts'
            cand -l 'Print the line counts'
            cand --lines 'Print the line counts'
            cand -L 'Print the maximum line width (Bytes)'
            cand --longest-line 'Print the maximum line width (Bytes)'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand all 'Enabled all available options'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'rwc;all'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
        &'rwc;help'= {
            cand all 'Enabled all available options'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'rwc;help;all'= {
        }
        &'rwc;help;help'= {
        }
    ]
    $completions[$command]
}
