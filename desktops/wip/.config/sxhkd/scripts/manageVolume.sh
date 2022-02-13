#!/bin/bash

VOLUME_STEP=2

if pactl get-default-sink | grep -i "bluez_sink" &>/dev/null; then
	VOLUME_STEP=1
fi

pamixer $1 $VOLUME_STEP
