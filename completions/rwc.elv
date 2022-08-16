
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
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand -b 'Print the count of bytes'
            cand --bytes 'Print the count of bytes'
            cand -c 'Print the count of chars'
            cand --chars 'Print the count of chars'
            cand -w 'Print the count of words'
            cand --words 'Print the count of words'
            cand -l 'Print the count of lines'
            cand --lines 'Print the count of lines'
            cand -L 'Print the length of the longest line'
            cand --longest-line 'Print the length of the longest line'
        }
    ]
    $completions[$command]
}
