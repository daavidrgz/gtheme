#!/bin/bash

bluetooth_connect() {
	if bluetoothctl show | grep -q "Powered: yes"; then
		devices_paired=$(bluetoothctl paired-devices | grep Device | cut -d ' ' -f 2)
		for device in $devices_paired; do
			bluetoothctl connect $device >> /dev/null
		done
	fi
}

bluetooth_print() {
	if bluetoothctl show | grep -q "Powered: yes"; then
        devices_paired=$(bluetoothctl paired-devices | grep Device | cut -d ' ' -f 2)
        device_alias=""

        for device in $devices_paired; do
            device_info=$(bluetoothctl info "$device")

            if echo "$device_info" | grep -q "Connected: yes"; then
                device_alias=$(echo "$device_info" | grep "Alias" | cut -d ' ' -f 2-)
            fi
        done

        if [ "$device_alias" == "" ]; then
            printf '  No device connected'
        else
            printf '  %s' "$device_alias"
        fi
	else
		printf '  Disconnected'
	fi
}

bluetooth_toggle() {
    if bluetoothctl show | grep -q "Powered: no"; then
        bluetoothctl power on >> /dev/null
    else
        devices_paired=$(bluetoothctl paired-devices | grep Device | cut -d ' ' -f 2)
        echo "$devices_paired" | while read -r line; do
            bluetoothctl disconnect "$line" >> /dev/null
        done

        bluetoothctl power off >> /dev/null
    fi
}

case "$1" in
    '--toggle')
        bluetooth_toggle;;
    '--connect')
		bluetooth_connect;;
    *)
        bluetooth_print;;
esac
