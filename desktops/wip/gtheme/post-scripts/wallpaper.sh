#!/bin/bash

Y="\e[1;33m"
W="\e[0m"
W_B="\e[1m"

WALLPAPER_URL=$1
feh --bg-fill "$WALLPAPER_URL"

[ "$2" == "nolightdm" ] && exit 0

# echo -e "${Y}•${W} Root privileges are needed in order to apply the wallpaper to lightdm webkit theme aether!\n"
LIGHTDM_WALLPAPER=/usr/share/lightdm-webkit/themes/lightdm-webkit-theme-aether/src/img/wallpapers/wallpaper
cp $WALLPAPER_URL $LIGHTDM_WALLPAPER
echo -e "Created wallpaper file in ${W_B}$LIGHTDM_WALLPAPER!${W}"

exit 0
