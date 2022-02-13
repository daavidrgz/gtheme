#/bin/bash -p

if cpupower frequency-info | grep "The governor \"powersave\""; then
	cpupower frequency-set -g performance
	sleep 1
	cpupower frequency-set --max 4100000
	sleep 1
	cpupower frequency-set --min 3900000
	
else
	cpupower frequency-set -g powersave
	sleep 1
	cpupower frequency-set --min 800000
	sleep 1
	cpupower frequency-set --max 2400000
fi
