#!/bin/bash

VSCODETHEME="$1"
VSCODE_SETTINGS_FILE="$HOME/.config/Code/User/settings.json"
	
if [ ! -e $VSCODE_SETTINGS_FILE ]; then
	echo -e "${R}[!]${W} The vscode settings file ${W_B}$VSCODE_SETTINGS_FILE${W} does not exist\n"
	return 1
fi

[ -z "$VSCODETHEME" ] && return

sed -i "s|\"workbench.colorTheme\": \".*\"|\"workbench.colorTheme\": \"$VSCODETHEME\"|" $VSCODE_SETTINGS_FILE
echo -e "${B}[+]${W} Theme ${W_B}\"$VSCODETHEME\"${W} applied to Visual Studio Code!\n"
