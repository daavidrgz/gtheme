<div align="center"> <h1><strong>GTHEME</strong></h1> </div>
<div align="center">

A **blazingly fast** easy to use **dotfile and global theme manager** for *NIX systems written in Rust 🔥

<!-- ![Ferris](screenshots/ferris.svg) -->
![Gtheme](screenshots/gtheme.gif)

Demo using **wip** desktop. To check out more desktop configurations refer to [Gtheme-Desktops →](https://github.com/daavidrgz/gtheme-desktops)

</div>

#

<details open>
<summary><strong>&nbsp;O V E R V I E W &nbsp;👁️‍🗨️</strong></summary>
<br>

Its main goal is to **standarize and abstract hardware specific configurations** and change your desktop colors
with **+350 different themes** and wallpapers.

You can write some patterns with generic attributes, and then Gtheme will fill those values with the appropriate ones following the selected theme/settings.

The final result you get is that you can change between any desktop of your choice with a **single command**, and change the theme of all applications in a **centralized and automatized way**. Moreover, you can also install desktops from other people who had
already adapted their dotfiles to Gtheme and everything will work just fine.

To sum up, you can try your favourite [Unixporn](https://www.reddit.com/r/unixporn/) ricings 🍚 (if authors port them to Gtheme)
</details>

#

<details open>
<summary><strong>&nbsp;I N S T A L L A T I O N &nbsp;🛠</strong></summary>
<br>


First of all, you should clone this repo, run the installation script and follow the initial setup:

```console
git clone https://github.com/daavidrgz/gtheme.git gtheme
cd gtheme
./install.sh
```

At the end, gtheme files should be placed in `~/.config/gtheme`, where you can see a `themes` and a `desktop` folder
where all color schemes and dotfiles are stored, respectively. You may see also a `wallpapers` folder if you
choose to download them.

Besides, your system's settings will be loaded from `~/.config/gtheme/user_settings.toml`. You can edit that file 
with `gtheme config edit` or run the setup again with `gtheme config setup`. This file is very important in order to be hardware agnostic and enjoy a full **Plug and Play** experience.
</details>

#

<details open>
<summary><strong>&nbsp;U S A G E &nbsp;🪧</strong></summary>

* **Text User Interface**

	* If you prefer a Text UI rather than a CLI, execute `gtheme` without arguments. 
	* To navigate between desktops and patterns to fav-themes and themes, press `tab`.
	* In order to see Text UI help and all included functionalities, press `h`.

* **Command Line Interface**

	You can see gtheme's help with `gtheme --help` and `gtheme <subcommand> --help`

	Here are some examples:

	- `gtheme desktop list`: will show all the desktops you've installed.
	- `gtheme desktop add <path/to/desktop>`: will install the specified desktop on `~/.config/gtheme/desktops/<desktop>`.
	- `gtheme theme list`: will show all the themes installed.
	- `gtheme desktop set-default-theme <theme> -d <desktop>`: will set a default theme for the specified desktop.
	- `gtheme desktop apply <desktop>`:  will apply the specified desktop and copy desktop's dotfiles to your `~/.config` folder.
	- `gtheme theme apply <theme>`: will apply the specified theme for the current desktop.


	> Command and themes/patterns/desktops completions should be available without any problems. If not, please submit an issue. 
	> Most of the subcommands has an alias for convenience. For example, you can run `gtheme desktop apply <theme>` with `gtheme d a <theme>`, or `gtheme theme apply <theme>` with `gtheme t a <theme>`.
</details>

#
## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
