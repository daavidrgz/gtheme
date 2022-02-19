#!/bin/bash

Y="\e[1;33m"
W="\e[0m"
W_B="\e[1m"

WALLPAPER_URL="$1"
IS_LIGHTDM="$2"
IS_THEMED="$3"

WALLPAPER_THEME=/tmp/wallpaper-theme
IMAGE_THEMING_PATH=$HOME/.config/gtheme/wallpapers

if [ "$IS_THEMED" == "true" ]; then
	echo -e "Theming"
	java -jar $IMAGE_THEMING_PATH/ImageTheming.jar $WALLPAPER_URL -t=$WALLPAPER_THEME -o=$IMAGE_THEMING_PATH

	declare OUTPUT_NAME=$(basename $WALLPAPER_URL | sed 's/\..*//')
	declare OUTPUT_EXTENSION=$(basename $WALLPAPER_URL | sed 's/.*\.//')
	WALLPAPER_URL="$IMAGE_THEMING_PATH/$OUTPUT_NAME-custom.$OUTPUT_EXTENSION"
fi

feh --bg-fill "$WALLPAPER_URL"

[ "$IS_LIGHTDM" == "false" ] && exit 0

# echo -e "${Y}•${W} Root privileges are needed in order to apply the wallpaper to lightdm webkit theme aether!\n"
LIGHTDM_WALLPAPER=/usr/share/lightdm-webkit/themes/lightdm-webkit-theme-aether/src/img/wallpapers/wallpaper
cp $WALLPAPER_URL $LIGHTDM_WALLPAPER
echo -e "Created wallpaper file in ${W_B}$LIGHTDM_WALLPAPER!${W}"

exit 0
