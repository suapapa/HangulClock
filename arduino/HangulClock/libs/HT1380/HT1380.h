/*
 * HT1380.h - Arduino library for RTC HT1380/HT1381 - header for C++ adaptation
 * by Homin Lee (Suapapa) <http://www.suapapa.net>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3
 * as published by the Free Software Foundation. You should have received
 * a copy of the license along with this program.
 * If not, see <http://www.gnu.org/licenses/>.
*/

#include "WConstants.h"

#ifndef HT1380_H
#define HT1380_H
class HT1380
{
	public:
		HT1380(uint8_t rest, uint8_t io, uint8_t sclk);

		void init(void);

		void readHour(void);
		void readMin(void);
		void readSec(void);
		void readYear(void);
		void readMonth(void);
		void readDate(void);
		void readDay(void);
		void readWP(void);
		void readBurst(void);

		void writeHour(void);
		void writeMin(void);
		void writeSec(void);
		void writeYear(void);
		void writeMonth(void);
		void writeDate(void);
		void writeDay(void);
		void writeWP(void);
		void writeBurst(void);

		uint8_t getHour(void);
		uint8_t getMin(void);
		uint8_t getSec(void);
		uint8_t getYear(void);
		uint8_t getMonth(void);
		uint8_t getDate(void);
		uint8_t getDay(void);
		uint8_t getWP(void);

		void setHour(uint8_t hour);
		void setMin(uint8_t min);
		void setSec(uint8_t sec);
		void setYear(uint8_t year);
		void setMonth(uint8_t month);
		void setDate(uint8_t date);
		void setDay(uint8_t day);
		void setWP(uint8_t wp);

	private:
		uint8_t _pinSCLK;
		uint8_t _pinIO;
		uint8_t _pinREST;

		uint8_t _hour;
		uint8_t _min;
		uint8_t _sec;
		uint8_t _year;
		uint8_t _month;
		uint8_t _date;
		uint8_t _day;
		uint8_t _wp;

		uint8_t	_readByte(void);
		void	_writeByte(uint8_t inByte);
		void	_ioStart(void);
		void	_ioEnd(void);

		uint8_t	_readRegistor(uint8_t regCmd);
		void	_writeRegistor(uint8_t regCmd, uint8_t regValue);

		uint8_t _parseReg2Num(uint8_t inReg);
		uint8_t _parseNum2Reg(uint8_t inNum);
};

#endif
