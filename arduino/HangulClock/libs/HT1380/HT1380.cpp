/*
 * HT1380.cpp - Arduino library for RTC HT1380/HT1381 - C++ adaptation
 * by Homin Lee (Suapapa) <http://www.suapapa.net>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3
 * as published by the Free Software Foundation. You should have received
 * a copy of the license along with this program.
 * If not, see <http://www.gnu.org/licenses/>.
*/

#include "HT1380.h"

#define CMD_SECOND	0x80
#define CMD_MINUTES	0x82
#define CMD_HOURS	0x84
#define CMD_DATE	0x86
#define CMD_MONTH	0x88
#define CMD_DAY		0x8A
#define CMD_YEAR	0x8C
#define CMD_WP		0x8E
#define CMD_BURST	0xBE

#define CMDMASK_WRITE	0x00
#define CMDMASK_READ	0x01

HT1380::HT1380(uint8_t rest, uint8_t io, uint8_t sclk)
{
	_pinSCLK = sclk;
	_pinIO = io;
	_pinREST = rest;

	pinMode(_pinSCLK, OUTPUT);
	pinMode(_pinREST, OUTPUT);
	pinMode(_pinIO, OUTPUT);

	_hour = 0;	_min = 0;	_sec = 0;
	_year = 0;	_month = 1;	_date = 1;
	_day = 1;	_wp = 0;
}

void HT1380::init(void)
{
	// disable the write protect (_WP=0)
	// and enable the oscillator (CH=0)
	_writeRegistor(CMD_WP, 0x00);
	_writeRegistor(CMD_SECOND, 0x00);
	// wait 3 seconds for ocillator generate
	// the clock for internal use
	delay(3000);
}

void HT1380::readHour(void)
{
	_hour = _parseReg2Num(_readRegistor(CMD_HOURS));
}

void HT1380::readMin(void)
{
	_min = _parseReg2Num(_readRegistor(CMD_MINUTES));
}

void HT1380::readSec(void)
{
	_min = _parseReg2Num(_readRegistor(CMD_SECOND));
}


void HT1380::readYear(void)
{
	uint8_t rawYear = _readRegistor(CMD_YEAR);
	_year = (rawYear & 0x0F);
	_year += (rawYear >> 4) * 10;
}

void HT1380::readMonth(void)
{
	_month = _parseReg2Num(_readRegistor(CMD_MONTH));
}

void HT1380::readDate(void)
{
	_date = _parseReg2Num(_readRegistor(CMD_DATE));
}

void HT1380::readDay(void)
{
	_day = _parseReg2Num(_readRegistor(CMD_DAY));
}

void HT1380::readWP(void)
{
	uint8_t rawWP = _readRegistor(CMD_WP);
	_wp = (rawWP >> 7);
}

void HT1380::readBurst(void)
{
	_ioStart();
	_writeByte(CMD_BURST|CMDMASK_READ);
	_sec = _parseReg2Num(_readByte());
	_min = _parseReg2Num(_readByte());
	_hour = _parseReg2Num(_readByte());
	_date = _parseReg2Num(_readByte());
	_month = _parseReg2Num(_readByte());
	_day = _parseReg2Num(_readByte());
	uint8_t rawYear = _readByte();
	_year = (rawYear & 0x0F);
	_year += (rawYear >> 4) * 10;
	uint8_t rawWP = _readByte();
	_wp = (rawWP >> 7);
	_ioEnd();
}

void HT1380::writeHour(void)
{
	_writeRegistor(CMD_HOURS, _parseNum2Reg(_hour));
}

void HT1380::writeMin(void)
{
	_writeRegistor(CMD_MINUTES, _parseNum2Reg(_min));
}

void HT1380::writeSec(void)
{
	_writeRegistor(CMD_SECOND, _parseNum2Reg(_sec));
}

void HT1380::writeYear(void)
{
	uint8_t regYear = _year % 10;
	regYear += ((_year / 10) << 4);
	_writeRegistor(CMD_YEAR, regYear);
}

void HT1380::writeMonth(void)
{
	_writeRegistor(CMD_MONTH, _parseNum2Reg(_month));
}

void HT1380::writeDate(void)
{
	_writeRegistor(CMD_DATE, _parseNum2Reg(_date));
}

void HT1380::writeDay(void)
{
	_writeRegistor(CMD_DAY, _parseNum2Reg(_day));
}

void HT1380::writeWP(void)
{
	uint8_t regWP = (_wp << 7);
	_writeRegistor(CMD_WP, regWP);
}

void HT1380::writeBurst(void)
{
	_ioStart();
	_writeByte(CMD_BURST);
	_writeByte(_parseNum2Reg(_sec));
	_writeByte(_parseNum2Reg(_min));
	_writeByte(_parseNum2Reg(_hour));
	_writeByte(_parseNum2Reg(_date));
	_writeByte(_parseNum2Reg(_month));
	_writeByte(_parseNum2Reg(_day));
	uint8_t regYear = _year % 10;
	regYear += ((_year / 10) << 4);
	_writeByte(regYear);
	_writeByte((_wp << 7));
	_ioEnd();
}

uint8_t HT1380::getHour(void)
{
	return _hour;
}

uint8_t HT1380::getMin(void)
{
	return _min;
}

uint8_t HT1380::getSec(void)
{
	return _sec;
}

uint8_t HT1380::getYear(void)
{
	return _year;
}

uint8_t HT1380::getMonth(void)
{
	return _month;
}

uint8_t HT1380::getDate(void)
{
	return _date;
}

uint8_t HT1380::getDay(void)
{
	return _day;
}

uint8_t HT1380::getWP(void)
{
	return _wp;
}

void HT1380::setHour(uint8_t hour)
{
	if (hour<24)
		_hour = hour;
	else
		_hour = 0;
}

void HT1380::setMin(uint8_t min)
{
	if (min<60)
		_min = min;
	else
		_min = 0;
}

void HT1380::setSec(uint8_t sec)
{
	if (sec<60)
		_sec = sec;
	else
		_sec =0;
}

void HT1380::setYear(uint8_t year)
{
	if (year<100)
		_year = year;
	else
		_year = 0;
}

void HT1380::setMonth(uint8_t month)
{
	if(month <= 12 && month > 0)
		_month = month;
	else
		_month = 1;
}

void HT1380::setDate(uint8_t date)
{
	if(date <= 31 && date > 0)
		_date = date;
	else
		_date = 1;
}

void HT1380::setDay(uint8_t day)
{
	if(day <= 7 && day > 0)
		_day = day;
	else
		_day = 1;
}

void HT1380::setWP(uint8_t wp)
{
	if(wp <= 1)
		_wp = wp;
	else
		_wp = 0;
}

/// receives a single byte from HT1380
uint8_t HT1380::_readByte(void)
{
	uint8_t i;
	uint8_t outByte = 0x00;
	pinMode(_pinIO, INPUT);
	for (i = 0; i < 8; i++)
	{
		digitalWrite(_pinSCLK, LOW);
		if (digitalRead(_pinIO) == HIGH)
			outByte |= (0x01<<i);
		digitalWrite(_pinSCLK, HIGH);
	}
	pinMode(_pinIO, OUTPUT);
	return outByte;
}

/// sends a single byte to HT1380
void HT1380::_writeByte(uint8_t inByte)
{
	uint8_t i;
	uint8_t mask;
	for (i = 0; i < 8; i++)
	{
		mask = 0x01 << i;
		digitalWrite(_pinSCLK, LOW);
		if (inByte & mask)
			digitalWrite(_pinIO, HIGH);
		else
			digitalWrite(_pinIO, LOW);
		digitalWrite(_pinSCLK, HIGH);
	}
	//delay 60 ns // Clock to Reset Hold(TCCH)
}

/// Set REST pin from low to high
void HT1380::_ioStart(void)
{
	digitalWrite(_pinREST, HIGH);
	digitalWrite(_pinSCLK, LOW);
	delayMicroseconds(1); // Reset to Clock Setup(TCC)
}

/// Set REST pin from high to low
void HT1380::_ioEnd(void)
{
	digitalWrite(_pinREST, LOW);
	digitalWrite(_pinSCLK, LOW);
	delayMicroseconds(1); // Reset Inactive Time(TCWH)
}

uint8_t	HT1380::_readRegistor(uint8_t regCmd)
{
	_ioStart();
	_writeByte(regCmd|CMDMASK_READ);
	uint8_t retByte = _readByte();
	_ioEnd();
	return retByte;
}

void HT1380::_writeRegistor(uint8_t regCmd, uint8_t regValue)
{
	_ioStart();
	_writeByte(regCmd);
	_writeByte(regValue);
	_ioEnd();
}

uint8_t HT1380::_parseReg2Num(uint8_t inReg)
{
	uint8_t retNum = 0x00;
	retNum += (inReg & 0x0F);
	retNum += ((inReg & 0x70) >> 4)*10;
	return retNum;
}

uint8_t HT1380::_parseNum2Reg(uint8_t inNum)
{
	uint8_t retReg = 0x00;
	retReg |= (inNum % 10);
	retReg |= ((inNum / 10) << 4);
	return retReg;
}
