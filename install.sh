#!/bin/bash

# Colors
R="\e[1;31m"
G="\e[1;32m"
Y="\e[1;33m"
B="\e[1;34m"
M="\e[1;35m"
C="\e[1;36m"
W="\e[0m"
W_B="\e[1m"

GTHEME_PATH=$HOME/.config/gtheme
BACKUP_PATH=$GTHEME_PATH/backup

function gthemeLogo() {
  echo -e "${R} ██████╗ ${G} ████████╗${Y} ██╗  ██╗${B} ███████╗${M} ███╗   ███╗${C} ███████╗${W}"
  echo -e "${R}██╔════╝ ${G} ╚══██╔══╝${Y} ██║  ██║${B} ██╔════╝${M} ████╗ ████║${C} ██╔════╝${W}"
  echo -e "${R}██║  ███╗${G}    ██║   ${Y} ███████║${B} █████╗  ${M} ██╔████╔██║${C} █████╗  ${W}"
  echo -e "${R}██║   ██║${G}    ██║   ${Y} ██╔══██║${B} ██╔══╝  ${M} ██║╚██╔╝██║${C} ██╔══╝  ${W}"
  echo -e "${R}╚██████╔╝${G}    ██║   ${Y} ██║  ██║${B} ███████╗${M} ██║ ╚═╝ ██║${C} ███████╗${W}"
  echo -e "${R} ╚═════╝ ${G}    ╚═╝   ${Y} ╚═╝  ╚═╝${B} ╚══════╝${M} ╚═╝     ╚═╝${C} ╚══════╝${W}"
	echo -e "\n"
}

function copyFiles() {
	echo -e "${G}->${W} Creating main gtheme folder in ${W_B}$GTHEME_PATH${W}..."
	mkdir $GTHEME_PATH &>/dev/null

	declare -a GTHEME_FOLDERS=("desktops" "themes")
	
	for FOLDER in ${GTHEME_FOLDERS[@]}; do
		declare REPO_NAME="gtheme-"$FOLDER
		declare REPO="https://github.com/daavidrgz/$REPO_NAME.git"

		echo -e "${G}->${W} Cloning ${W_B}$REPO_NAME${W}..."
		rm -rf /tmp/$REPO_NAME &>/dev/null
		git clone --depth=1 $REPO /tmp/$REPO_NAME || echo -e "${R}->${W} There was an error while cloning ${W_B}$FOLDER/${W}!\n"
		rm -rf /tmp/$REPO_NAME/.git /tmp/$REPO_NAME/desktops.gif &>/dev/null
		echo -e "${G}->${W} Transfering ${W_B}$REPO_NAME${W}..."
		mv /tmp/$REPO_NAME $GTHEME_PATH/$FOLDER &>/dev/null || echo -e "${R}->${W} There was an error while copying ${W_B}$FOLDER/${W}!\n"
	done
	mv $GTHEME_PATH/themes/themes/* $GTHEME_PATH/themes
	mv $GTHEME_PATH/themes/global_config.json $GTHEME_PATH
	rmdir $GTHEME_PATH/themes/themes
	echo -e "${G}-> Done!${W}"	
}

function backupConfig() {
	AVAIL_SIZE=$(df -P / | awk 'END{print $4}')
	FOLDER_SIZE=$(du -k $HOME/.config -d 0 | awk '{print $1}')	
	if [ "$FOLDER_SIZE" -gt "$AVAIL_SIZE" ]; then
		echo -e "${R}-> Error${W}, there is no enough space in ${W_B}/tmp${W}\n"
		exit 2
	fi

	echo -e "\n${G}->${W} Copying all your files. This may take a while..."
	cp -r $HOME/.config /tmp/current-config
	[ ! -e $BACKUP_PATH ] && mkdir -p $BACKUP_PATH
	mv /tmp/current-config/* $BACKUP_PATH
	echo -e "${G}-> Backup done!${W}"
}

function installWallpapers() {
	echo -e "${G}->${W} Cloning gtheme-wallpapers repository. This may take a while..."
	rm -rf /tmp/gtheme-wallpapers &>/dev/null
	git clone --depth=1 https://github.com/daavidrgz/gtheme-wallpapers.git /tmp/gtheme-wallpapers
	rm -rf /tmp/gtheme-wallpapers/.git
	mv /tmp/gtheme-wallpapers $GTHEME_PATH/wallpapers
	echo -e "${G}->${W} Wallpapers succesfully installed!"
}

function askBackup() {
	declare -r CONFIG_SIZE=$(du -ha -d 0 $HOME/.config | awk '{print $1}') 
	while true; do
		echo -en "${B}->${W} Do you want to make a backup? All your ${W_B}$HOME/.config${W} folder will be copied to ${W_B}$BACKUP_PATH${W} [~$CONFIG_SIZE] ${G}(y/[N])${W} "
		read INPUT
		case $INPUT in 
			y | Y) 
				backupConfig
				return 0;;
			n | N | "")
				echo -e "${Y}->${W} Skipping backup creation..."
				return 0;;
			*)
				echo -e "\n${R}->${W} Incorrect option!\n";;
		esac
	done
}

function askCopy() {
	while true; do
		echo -e "${Y}->${W} It looks like you have already installed gtheme. Do you want to reinstall it?"
		echo -en "(this will potentially override some files in ${Y}$GTHEME_PATH${W}) ${G}(y/[N])${W} "

		read INPUT
		case $INPUT in 
			y | Y)
				copyFiles
				return 0;;
			n | N | "")
				echo -e "${Y}->${W} Skipping files copy..."
				return 0;;
			*)
				echo -e "\n${R}->${W} Incorrect option!\n";;
		esac
	done
}

function askWallpapers() {
		while true; do
		echo -en "${G}->${W} Do you want to download ${G}gtheme-wallpapers${W}? (~350MB) ${G}([Y]/n)${W} "
		read INPUT
		case $INPUT in 
			y | Y | "")
				installWallpapers
				return 0;;
			n | N)
				echo -e "${Y}->${W} Skipping wallpapers download..."
				return 0;;
			*)
				echo -e "\n${R}->${W} Incorrect option!\n";;
		esac
	done
}

function install() {
	echo -e "${G}->${W} Compiling program..."
	cargo build --release || exit 1
	
	clear
	gthemeLogo
	echo -e "${G}->${W} Copying binary to ${W_B}/usr/bin${W}..."
	echo -e "${G}->${W} You must be root to proceed!"
	sudo cp target/release/gtheme /usr/bin || echo -e "${R}->${W} There was an error while copying script to ${W_B}/usr/bin${W}\n"

	echo -e "${G}->${W} Setting up autocompletion scripts..."
	if [ -e ~/.zshrc ]; then
		if ! cat ~/.zshrc | grep 'fpath=(~/.config/gtheme/completions $fpath)' &>/dev/null; then
			echo -e '\nfpath=(~/.config/gtheme/completions $fpath)\nautoload -Uz compinit && compinit' >> ~/.zshrc
		fi
	fi
	if [ -e ~/.bashrc ]; then
		if ! cat ~/.bashrc | grep '[ -r ~/.config/gtheme/completions/gtheme.bash ]' &>/dev/null; then
			echo -e '\n[ -r ~/.config/gtheme/completions/gtheme.bash ] && source ~/.config/gtheme/completions/gtheme.bash' >> ~/.bashrc
		fi
	fi
	echo -e "${G}-> Done!${W}"
	echo -e "${W_B}(You need to restart your shell in order to work properly)${W}\n"

	if [ -e "$GTHEME_PATH" ]; then
		askCopy
	else
		askBackup; echo; copyFiles
	fi

	echo; askWallpapers; echo
}

function cleanFiles() {
	rm -rf $dir && echo -e "${G}->${W} Succesfully removed $dir/" || echo -e "${R}->${W} Error while removing $dir/"
	echo -e "${G}-> Done!${W}"
}

function main() {
	cd $(realpath $(dirname $0))
	clear; gthemeLogo
	install
	
	/usr/bin/gtheme config setup

	clear; gthemeLogo
	echo -e "${G}->${W} Cleaning installation files..."
	# cleanFiles

	echo -e "\n${B}-> Installation finished!${W}\n"
	echo -e "${B}->${W} To get more information about gtheme usage refer to the repo: ${B}https://github.com/daavidrgz/gtheme${W}"
	echo -e "${B}->${W} Feel free to also check my dotfiles: ${B}https://github.com/daavidrgz/dotfiles${W}\n"

	exit 0
}

main 
