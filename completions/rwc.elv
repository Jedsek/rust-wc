
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
            cand -b 'Show the count of bytes'
            cand --bytes 'Show the count of bytes'
            cand -c 'Show the count of chars'
            cand --chars 'Show the count of chars'
            cand -w 'Show the count of words'
            cand --words 'Show the count of words'
            cand -l 'Show the count of lines'
            cand --lines 'Show the count of lines'
            cand -L 'Show the length of the longest line'
            cand --longest-line 'Show the length of the longest line'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
        }
    ]
    $completions[$command]
}
