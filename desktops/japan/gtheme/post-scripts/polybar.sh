#!/bin/bash

while pgrep -u $UID -x polybar &>/dev/null; do
	killall -q -s KILL polybar
	sleep 1
done

polybar main & disown

exit 0
