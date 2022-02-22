#!/bin/bash

VOLUME_STEP=2

if pactl get-default-sink | grep -i "bluez_sink" &>/dev/null; then
	VOLUME_STEP=1
fi

pamixer $1 $VOLUME_STEP \
&& dunstify -a System -t 1000 -h string:x-dunst-stack-tag:volume -h int:value:$(pamixer --get-volume) "Volume: $(pamixer --get-volume)%"
