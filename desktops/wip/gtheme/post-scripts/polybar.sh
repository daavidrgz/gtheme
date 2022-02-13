#!/bin/bash

while pgrep -u $UID -x polybar &>/dev/null; do
	killall -q polybar
	sleep 1
done
nohup polybar -q main -c $HOME/.config/polybar/config.ini &
disown %1

exit 0
