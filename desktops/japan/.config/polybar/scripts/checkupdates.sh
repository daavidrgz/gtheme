#!/bin/sh

updates_arch=$(checkupdates 2>/dev/null | wc -l ) || updates_arch=0
updates_aur=$(yay -Qum 2>/dev/null | wc -l) || updates_aur=0

updates=$(($updates_arch + $updates_aur))

echo $updates
