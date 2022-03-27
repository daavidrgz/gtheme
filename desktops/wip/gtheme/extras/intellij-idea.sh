#!/bin/bash

IDEA_THEME="$1"
IDEA_FOLDER="$HOME/.config/JetBrains"
[ -z "$IDEA_THEME" ] && exit 1

for dir in $IDEA_FOLDER/*; do
	echo $dir | grep -qi "idea" || continue
	sed -i "s|global_color_scheme name=\".*\"|global_color_scheme name=\"$IDEA_THEME\"|" $dir/options/colors.scheme.xml
done
