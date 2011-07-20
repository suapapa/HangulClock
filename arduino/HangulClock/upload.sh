#!/bin/bash

scons ARDUINO_BOARD=atmega8 ARDUINO_PORT=/dev/ttyUSB1 EXTRA_LIB=libs upload
