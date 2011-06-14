#include <HT1380.h>
/*
 * HT1380_example - Demonstrates the use of the HT1380 library
 * by Homin Lee (Suapapa) <http://www.suapapa.net>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 3
 * as published by the Free Software Foundation. You should have received
 * a copy of the license along with this program.
 * If not, see <http://www.gnu.org/licenses/>.
*/

HT1380 rtc = HT1380(7, 6, 5);

void setup()
{
  Serial.begin(9600);

  rtc.init();
  Serial.println("rtc HT1380 now ready!"); 

  // set current time to RTC instance
  rtc.setHour(19);
  rtc.setMin(9);
  rtc.setSec(0);
  rtc.setYear(8);
  rtc.setMonth(8);
  rtc.setDate(19);
  rtc.setDay(3);
  rtc.setWP(1);

  // write the time to HT1380
  rtc.writeBurst();
}

void loop()
{
  delay(1000);

  // read current time from HT1380
  rtc.readBurst();

  // show it
  Serial.print((int)rtc.getHour());
  Serial.print(":");
  Serial.print((int)rtc.getMin());
  Serial.print(":");
  Serial.println((int)rtc.getSec());
}
