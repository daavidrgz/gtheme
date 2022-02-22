#!/bin/bash
THEMES_PATH=$HOME"/.config/gtheme/themes"
DEST_PATH=$HOME"/github/gtheme/themes"

ATTR_NAMES=("background" "foreground" "cursor" "selection-background" "selection-foreground" \
"black" "black-hg" "red" "red-hg" "green" "green-hg" "yellow" "yellow-hg" "blue" "blue-hg" "magenta" "magenta-hg" "cyan" "cyan-hg" "white" "white-hg" "vscode" "wallpaper")

for f in $THEMES_PATH/*; do
  NAME=$(echo $(basename $f) | sed 's/.colors//')
	DEST=$DEST_PATH/$(echo $(basename $f) | sed 's/.colors/.json/g')
	cp $HOME/github/gtheme/utils/theme.json $DEST
  declare CONTENT="$(/bin/cat $f)"
	for attr in ${ATTR_NAMES[@]}; do
    declare VALUE=$(echo "$CONTENT" | grep -e "^$attr:.*" | awk -F ': ' '{print $2}')
    if [[ $attr != "wallpaper" && $attr != "vscode" ]]; then
      VALUE=${VALUE:0:6}
    fi
    sed -i "s|%$attr%|$VALUE|g" $DEST
  done
  sed -i "s|%name%|$NAME|g" $DEST
done
