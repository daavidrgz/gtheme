#!/bin/bash

WALLPAPER_URL="$1"
WALLPAPER_THEME=/tmp/wallpaper-theme
IMAGE_THEMING_PATH=$HOME/.config/gtheme/wallpapers

[ -z "$WALLPAPER_URL" ] && exit 1

if [ "$IS_THEMED" == "true" ]; then
	java -jar $IMAGE_THEMING_PATH/ImageTheming.jar $WALLPAPER_URL -t=$WALLPAPER_THEME -o=$IMAGE_THEMING_PATH

	declare OUTPUT_NAME=$(basename $WALLPAPER_URL | sed 's/\..*//')
	declare OUTPUT_EXTENSION=$(basename $WALLPAPER_URL | sed 's/.*\.//')
	WALLPAPER_URL="$IMAGE_THEMING_PATH/$OUTPUT_NAME-custom.$OUTPUT_EXTENSION"
fi

cp "$WALLPAPER_URL" ~/.wallpaper
feh --bg-fill "$WALLPAPER_URL"
