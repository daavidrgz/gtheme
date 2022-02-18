#!/bin/bash

DEST=$1
kitty @ set-colors --all $DEST &> /dev/null
tput sgr0
tput op
exit 0
