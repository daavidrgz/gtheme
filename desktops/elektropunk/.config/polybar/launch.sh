#!/usr/bin/env sh

## Add this to your wm startup file.

# Terminate already running bar instances
killall -q polybar

# Wait until the processes have been shut down
while pgrep -u $UID -x polybar >/dev/null; do sleep 1; done

# Launch main bar
if type "xrandr"; then
  polybar --reload main --config=~/.config/polybar/split-bar.ini &
else
  polybar main --config=~/.config/polybar/split-bar.ini &
fi
