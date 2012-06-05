#!/usr/bin/python
# -*- coding: utf-8 -*-
 
# set_current_time.py - description
#
# Copyright (C) 2011 Homin Lee <ff4500@gmail.com>
#
# This program is free software; you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation; either version 2 of the License, or
# (at your option) any later version.

from subprocess import Popen, PIPE
import serial

def getCurrentTimeStr():
    cmd = r"date +%H%M%S"
    p = Popen(cmd, shell=True, stdout=PIPE, stderr=PIPE)
    stdout, stderr = p.communicate()
    return stdout

    packet = "#S"+stdout
    return packet

if __name__ == "__main__":
    import sys

    if len(sys.argv) == 1:
        packet = "#S" + getCurrentTimeStr();
    else:
        hour, minute = int(sys.argv[1][:2]), int(sys.argv[1][2:])
        if hour < 0 or hour > 24 or minute < 0 or minute > 60:
            print "Bad input time. You input %s. means hour %d, minute %d"%
                        (sys.argv[0], hour, minute)
        packet = "#S%02d%02d"%(hour, minute)

    print "Sending", packet, "..."

    ser = serial.Serial("/dev/ttyUSB0", 9600, timeout=1)
    ser.write(packet)
    ser.flush()
    print ser.read(2)
    ser.flush()
    ser.close()

# vim: et sw=4 fenc=utf-8:

