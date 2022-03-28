#!/bin/bash

VSCODETHEME="$1"
VSCODE_SETTINGS_FILE="$HOME/.config/Code/User/settings.json"
	
if [ ! -e $VSCODE_SETTINGS_FILE ]; then
	echo -e "[!] The vscode settings file $VSCODE_SETTINGS_FILE$ does not exist\n"
	return 1
fi

[ -z "$VSCODETHEME" ] && exit 1

sed -i "s|\"workbench.colorTheme\": \".*\"|\"workbench.colorTheme\": \"$VSCODETHEME\"|" $VSCODE_SETTINGS_FILE
echo -e "[+] Theme \"$VSCODETHEME\" applied to Visual Studio Code!\n"
