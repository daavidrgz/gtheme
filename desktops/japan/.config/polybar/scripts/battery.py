#!/usr/bin/python
import psutil

percent = psutil.sensors_battery().percent
print(int(percent))
