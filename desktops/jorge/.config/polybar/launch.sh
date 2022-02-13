#!/usr/bin/env bash

while pgrep -u $UID -x polybar >/dev/null; do
	killall -q polybar
	sleep 1
done
polybar -r -q main -c $HOME/.config/polybar/config.ini &
