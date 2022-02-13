#!/bin/bash

CORES=8
FREQ_SUM="$(sudo cpupower monitor -m "Mperf" | awk -F '|' '{s+=$NF} END {print s}')"

echo "scale=2; $FREQ_SUM / $CORES / 1000" | bc | awk '{printf "%.2f GHz", $0}'

# if [ "$(cpupower frequency-info | grep "The governor \"powersave\"")" == "" ]; then
#   printf " ()"
# else
# 	printf " ()"
# fi
