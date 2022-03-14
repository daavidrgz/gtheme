
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'gtheme' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'gtheme'
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
        'gtheme' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('theme', 'theme', [CompletionResultType]::ParameterValue, 'Manage themes')
            [CompletionResult]::new('desktop', 'desktop', [CompletionResultType]::ParameterValue, 'Manage desktops')
            [CompletionResult]::new('pattern', 'pattern', [CompletionResultType]::ParameterValue, 'Manage patterns')
            [CompletionResult]::new('extra', 'extra', [CompletionResultType]::ParameterValue, 'Manage extras')
            [CompletionResult]::new('fav', 'fav', [CompletionResultType]::ParameterValue, 'Manage fav themes')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'gtheme;theme' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all installed themes')
            [CompletionResult]::new('apply', 'apply', [CompletionResultType]::ParameterValue, 'Apply specified theme')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'gtheme;theme;list' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;theme;apply' {
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Apply the theme only on selected patterns')
            [CompletionResult]::new('--pattern', 'pattern', [CompletionResultType]::ParameterName, 'Apply the theme only on selected patterns')
            [CompletionResult]::new('-i', 'i', [CompletionResultType]::ParameterName, 'Invert the foreground and background colors on selected patterns')
            [CompletionResult]::new('--invert', 'invert', [CompletionResultType]::ParameterName, 'Invert the foreground and background colors on selected patterns')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;theme;help' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;desktop' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all installed desktops')
            [CompletionResult]::new('install', 'install', [CompletionResultType]::ParameterValue, 'Install specified desktop')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'gtheme;desktop;list' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;desktop;install' {
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Apply specified theme after installing the desktop')
            [CompletionResult]::new('--theme', 'theme', [CompletionResultType]::ParameterName, 'Apply specified theme after installing the desktop')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;desktop;help' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;pattern' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all patterns of the current desktop by default')
            [CompletionResult]::new('enable', 'enable', [CompletionResultType]::ParameterValue, 'Enable specified patterns in the current desktop')
            [CompletionResult]::new('disable', 'disable', [CompletionResultType]::ParameterValue, 'Disable specified patterns in the current desktop')
            [CompletionResult]::new('toggle', 'toggle', [CompletionResultType]::ParameterValue, 'Toggle specified patterns in the current desktop')
            [CompletionResult]::new('invert', 'invert', [CompletionResultType]::ParameterValue, 'Invert specified patterns or return them to default if they are already inverted')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'gtheme;pattern;list' {
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'List patterns of the specified desktop')
            [CompletionResult]::new('--desktop', 'desktop', [CompletionResultType]::ParameterName, 'List patterns of the specified desktop')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;pattern;enable' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;pattern;disable' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;pattern;toggle' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;pattern;invert' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;pattern;help' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;extra' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('enable', 'enable', [CompletionResultType]::ParameterValue, 'Enable specified extras in the current desktop')
            [CompletionResult]::new('disable', 'disable', [CompletionResultType]::ParameterValue, 'Disable specified extras in the current desktop')
            [CompletionResult]::new('toggle', 'toggle', [CompletionResultType]::ParameterValue, 'Toggle specified extras in the current desktop')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'gtheme;extra;enable' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;extra;disable' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;extra;toggle' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;extra;help' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;fav' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List favourite themes')
            [CompletionResult]::new('add', 'add', [CompletionResultType]::ParameterValue, 'Add selected themes to the favourite themes list')
            [CompletionResult]::new('remove', 'remove', [CompletionResultType]::ParameterValue, 'Remove selected themes from the favourite themes list')
            [CompletionResult]::new('toggle', 'toggle', [CompletionResultType]::ParameterValue, 'Toggle selected themes from the favourite themes list')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'gtheme;fav;list' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;fav;add' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;fav;remove' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;fav;toggle' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;fav;help' {
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
        'gtheme;help' {
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Show more information')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Show more information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
