# Gtheme

Gtheme is a dotfile and global theme manager for *NIX systems. It is aimed to make your ricing life much better. 

You can write some patterns with generic attributes, and then Gtheme will fill those values with the appropriate ones following the selected theme/settings.

The final result you got is that you can change between any desktop of your choice with a single command, and change the theme of all applications in a centralized and automatized way. Moreover, you can also
install desktops from other people who had already adapted their dotfiles to gtheme and everything will work just fine.

## Installation
First of all, you should clone this repo, run the installation script and follow the setup.

```console
git clone https://github.com/daavidrgz/gtheme.git
cd gtheme
./install.sh
```

At the end, gtheme files should be placed in `~/.config/gtheme`, where you can see a `themes` folder with +300 different themes that you can use on your patterns and a `desktop` folder where dotfiles are stored.

Moreover, your system's settings will be loaded from `~/.config/gtheme/user_settings.toml`, you can edit that file or run the setup again with `gtheme setup`. This file is very important in order to be hardware agnostic and experience a full **Plug and Play** feeling.

## Usage

You can see gtheme's help with `gtheme --help` and `gtheme <subcommand> --help`

Here are some examples:

- `gtheme desktop list`: will show all the desktops you've installed.
- `gtheme desktop add <path/to/desktop>`: will install the specified desktop on `~/.config/gtheme/desktops/<desktop>`.
- `gtheme theme list`: will show all the themes installed.
- `gtheme desktop set-default-theme <theme> -d <desktop>`: will set a default theme for the specified desktop.
- `gtheme desktop apply <desktop>`:  will apply the specified desktop and copy desktop's dotfiles to your `~/.config` folder.
- `gtheme theme apply <theme>`: will apply the specified theme for the current desktop.

> Most of the subcommands has an alias for convenience. For example, you can run `gtheme desktop apply <theme>` with `gtheme d a <theme>`, or `gtheme theme apply <theme>` with `gtheme t a <theme>`.
