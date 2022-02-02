#!/bin/bash
THEMES_PATH=$HOME"/.config/gtheme-rust/themes"
DEST_PATH=$HOME"/.config/gtheme-rust/json-themes"

ATTR_NAMES=("background" "foreground" "cursor" "selection-background" "selection-foreground" \
"black" "black-hg" "red" "red-hg" "green" "green-hg" "yellow" "yellow-hg" "blue" "blue-hg" "magenta" "magenta-hg" "cyan" "cyan-hg" "white" "white-hg" "vscode" "wallpaper")

for f in $THEMES_PATH/*; do
	DEST=$DEST_PATH/$(echo $(basename $f) | sed 's/.colors/.json/g')
	cp $HOME/.config/gtheme-rust/theme.json $DEST
  declare CONTENT="$(/bin/cat $f)"
	for attr in ${ATTR_NAMES[@]}; do
    declare VALUE=$(echo "$CONTENT" | grep -e "^$attr:.*" | awk -F ': ' '{print $2}')
    sed -i "s|%$attr%|$VALUE|g" $DEST
  done
done
