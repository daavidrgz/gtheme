#!/bin/bash

killall polybar
polybar -q main -c $HOME/.config/polybar/config.ini
