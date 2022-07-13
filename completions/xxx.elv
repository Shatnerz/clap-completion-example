
use builtin;
use str;

set edit:completion:arg-completer[xxx] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'xxx'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'xxx'= {
            cand --host 'Attempting to get hostname completion to work'
            cand -h 'Print help information'
            cand --help 'Print help information'
        }
    ]
    $completions[$command]
}
