<div align="center"> <h1><strong>GTHEME</strong></h1> </div>
<div align="center">

A **blazingly fast** easy to use **dotfile and global theme manager** for *NIX systems written in Rust 🔥.

![Gtheme](screenshots/gtheme.gif)

</div>

#

<details open>
<summary><strong>&nbsp;O V E R V I E W &nbsp;👁️‍🗨️</strong></summary>
Its main goal is to standarize and abstract hardware specific configurations and change your desktop colors
with +350 different themes and wallpapers.

You can write some patterns with generic attributes, and then Gtheme will fill those values with the appropriate ones following the selected theme/settings.

The final result you get is that you can change between any desktop of your choice with a single command, and change the theme of all applications in a centralized and automatized way. Moreover, you can also
install desktops from other people who had already adapted their dotfiles to Gtheme and everything will work just fine.

AKA you can try your favourite [Unixporn](https://www.reddit.com/r/unixporn/ ricings 🍚 (if authors port them to Gtheme)
</details>

#

<details open>
<summary><strong>&nbsp;I N S T A L L A T I O N &nbsp;🛠</strong></summary>
First of all, you should clone this repo, run the installation script and follow the setup.

```console
git clone https://github.com/daavidrgz/gtheme.git gtheme
cd gtheme
./install.sh
```

At the end, gtheme files should be placed in `~/.config/gtheme`, where you can see a `themes` folder where al color schemes are saved and a `desktop` folder where dotfiles live.

Moreover, your system's settings will be loaded from `~/.config/gtheme/user_settings.toml`, you can edit that file 
with `gtheme config edit` or run the setup again with `gtheme setup`. This file is very important in order to be hardware agnostic and enjoy a full **Plug and Play** experience.
</details>

<details open>
<summary><strong>&nbsp;U S A G E &nbsp;🛠</strong></summary>

* **Terminal User Interface**

	* If you prefer a Terminal UI rather than a CLI, execute `gtheme` without arguments. 
	* To navigate between desktops and patterns to fav-themes and themes, press `tab`.
	* In order to see Terminal UI help and all included functionalities, press `h`.

<br>

* **Command Line Interface**

	You can see gtheme's help with `gtheme --help` and `gtheme <subcommand> --help`

	Here are some examples:

	- `gtheme desktop list`: will show all the desktops you've installed.
	- `gtheme desktop add <path/to/desktop>`: will install the specified desktop on `~/.config/gtheme/desktops/<desktop>`.
	- `gtheme theme list`: will show all the themes installed.
	- `gtheme desktop set-default-theme <theme> -d <desktop>`: will set a default theme for the specified desktop.
	- `gtheme desktop apply <desktop>`:  will apply the specified desktop and copy desktop's dotfiles to your `~/.config` folder.
	- `gtheme theme apply <theme>`: will apply the specified theme for the current desktop.

	> Most of the subcommands has an alias for convenience. For example, you can run `gtheme desktop apply <theme>` with `gtheme d a <theme>`, or `gtheme theme apply <theme>` with `gtheme t a <theme>`.
