#!/bin/bash

scons ARDUINO_BOARD=atmega8l ARDUINO_PORT=/dev/ttyUSB0 EXTRA_LIB=libs upload
