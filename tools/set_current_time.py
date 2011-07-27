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
cmd = r"date +%H%M%S"
p = Popen(cmd, shell=True, stdout=PIPE, stderr=PIPE)
stdout, stderr = p.communicate()

packet = "#S"+stdout
print packet

import serial
ser = serial.Serial("/dev/ttyUSB0", 9600, timeout=1)
ser.write(packet)
ser.flush()
print ser.read(2)
ser.close()

# vim: et sw=4 fenc=utf-8:

