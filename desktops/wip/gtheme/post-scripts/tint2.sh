#!/bin/sh

killall -q tint2
nohup tint2 &
disown
exit 0
