#!/bin/bash

if pgrep paperview &>/dev/null; then
	killall paperview
	~/.fehbg
else
	paperview ~/.config/paperview/japan 12 &
fi

exit 0
