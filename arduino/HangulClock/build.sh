#!/bin/bash

scons ARDUINO_BOARD=atmega8 ARDUINO_PORT=/dev/ttyUSB0 EXTRA_LIB=`realpath libs/` $1
