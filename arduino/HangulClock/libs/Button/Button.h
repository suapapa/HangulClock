/*
||
|| @file Button.h
|| @version 1.6
|| @author Alexander Brevig
|| @contact alexanderbrevig@gmail.com
||
|| @description
|| | Provide an easy way of making buttons
|| #
||
|| @license
|| | Copyright (c) 2009 Alexander Brevig. All rights reserved.
|| | This code is subject to AlphaLicence.txt
|| | alphabeta.alexanderbrevig.com/AlphaLicense.txt
---------------------( COPY )---------------------
This libraries .h file has been updated by Terry King to make it compatible with Ardino 1.0x 

(Example 1.03) and also earlier versions like 0023

#include "WProgram.h"  
...has been replaced by:

#if defined(ARDUINO) && ARDUINO >= 100
#include "Arduino.h"
#else
#include "WProgram.h"
#endif

terry@yourduino.com
02/07/2012
-----------------( END COPY )----------------------
|| #
||
*/

#ifndef BUTTON_H
#define BUTTON_H

#if defined(ARDUINO) && ARDUINO >= 100
#include "Arduino.h"
#else
#include "WProgram.h"
#endif

#define PULLUP HIGH
#define PULLDOWN LOW

#define CURRENT 0
#define PREVIOUS 1
#define CHANGED 2

class Button{
  public:
    Button(uint8_t buttonPin, uint8_t buttonMode=PULLDOWN);
    void pullup();
    void pulldown();
    bool isPressed();
    bool wasPressed();
    bool stateChanged();
	bool uniquePress();
  private: 
    uint8_t pin;
    uint8_t mode;
    uint8_t state;
};

#endif

/*
|| @changelog
|| | 1.6 2009-05-05 - Alexander Brevig : Added uniquePress, it returns true if the state has changed AND the button is pressed
|| | 1.5 2009-04-24 - Alexander Brevig : Added stateChanged, @contribution http://www.arduino.cc/cgi-bin/yabb2/YaBB.pl?action=viewprofile;username=klickadiklick
|| | 1.4 2009-04-24 - Alexander Brevig : Added wasPressed, @contribution http://www.arduino.cc/cgi-bin/yabb2/YaBB.pl?action=viewprofile;username=klickadiklick
|| | 1.3 2009-04-12 - Alexander Brevig : Added constructor with one parameter Button(uint8_t buttonPin)
|| | 1.2 2009-04-10 - Alexander Brevig : Namechange from Switch to Button
|| | 1.1 2009-04-07 - Alexander Brevig : Altered API
|| | 1.0 2008-10-23 - Alexander Brevig : Initial Release
|| #
*/


