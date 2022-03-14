
use builtin;
use str;

set edit:completion:arg-completer[gtheme] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'gtheme'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'gtheme'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -V 'Print version information'
            cand --version 'Print version information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
            cand theme 'Manage themes'
            cand desktop 'Manage desktops'
            cand pattern 'Manage patterns'
            cand extra 'Manage extras'
            cand fav 'Manage fav themes'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'gtheme;theme'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
            cand list 'List all installed themes'
            cand apply 'Apply specified theme'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'gtheme;theme;list'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;theme;apply'= {
            cand -p 'Apply the theme only on selected patterns'
            cand --pattern 'Apply the theme only on selected patterns'
            cand -i 'Invert the foreground and background colors on selected patterns'
            cand --invert 'Invert the foreground and background colors on selected patterns'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;theme;help'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;desktop'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
            cand list 'List all installed desktops'
            cand install 'Install specified desktop'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'gtheme;desktop;list'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;desktop;install'= {
            cand -t 'Apply specified theme after installing the desktop'
            cand --theme 'Apply specified theme after installing the desktop'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;desktop;help'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;pattern'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
            cand list 'List all patterns of the current desktop by default'
            cand enable 'Enable specified patterns in the current desktop'
            cand disable 'Disable specified patterns in the current desktop'
            cand toggle 'Toggle specified patterns in the current desktop'
            cand invert 'Invert specified patterns or return them to default if they are already inverted'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'gtheme;pattern;list'= {
            cand -d 'List patterns of the specified desktop'
            cand --desktop 'List patterns of the specified desktop'
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;pattern;enable'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;pattern;disable'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;pattern;toggle'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;pattern;invert'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;pattern;help'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;extra'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
            cand enable 'Enable specified extras in the current desktop'
            cand disable 'Disable specified extras in the current desktop'
            cand toggle 'Toggle specified extras in the current desktop'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'gtheme;extra;enable'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;extra;disable'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;extra;toggle'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;extra;help'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;fav'= {
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
            cand list 'List favourite themes'
            cand add 'Add selected themes to the favourite themes list'
            cand remove 'Remove selected themes from the favourite themes list'
            cand toggle 'Toggle selected themes from the favourite themes list'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'gtheme;fav;list'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;fav;add'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;fav;remove'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;fav;toggle'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;fav;help'= {
            cand --version 'Print version information'
            cand -h 'Print help information'
            cand --help 'Print help information'
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
        &'gtheme;help'= {
            cand -v 'Show more information'
            cand --verbose 'Show more information'
        }
    ]
    $completions[$command]
}
