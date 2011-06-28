#include <Sprite.h>
#include <Matrix.h>

#define PIN_LED 13

#define PIN_MAX7219_DATA 4  
#define PIN_MAX7219_CLK  3
#define PIN_MAX7219_LOAD 2
Matrix mat = Matrix(PIN_MAX7219_DATA, PIN_MAX7219_CLK, PIN_MAX7219_LOAD);

int col[5] = {3, 1, 5, 4, 0};
int row[5] = {7, 2, 6, 1, 5};
#define _M_ON(M, R, C) M.write(row[R], col[C], HIGH)
#define _M_OFF(M, R, C) M.write(row[R], col[C], LOW)

//#define M_ON(M, C, R) _M_ON(M, R, C)
//#define M_OFF(M, C, R) _M_OFF(M, R, C)
#define M_ON(M, R, C) _M_ON(M, R, C)
#define M_OFF(M, R, C) _M_OFF(M, R, C)

Sprite timePanel = Sprite(
    5, 5,
    B00000,
    B00000,
    B00000,
    B00000,
    B00000
    );
#define P_ON(C, R) M_ON(mat, R, C)
#define P_OFF(C, R) M_OFF(mat, R, C)

#define CLEAR_PANEL() \
  for(int i = 0; i < 5; i++) {\
    for(int j = 0; j < 5; j++) {\
      P_OFF(i, j);\
    }\
  }

#include <HT1380.h>

#define PIN_HT1380_SCLK 6
#define PIN_HT1380_IO 5
#define PIN_HT1380_REST 4
HT1380 rtc = HT1380(PIN_HT1380_REST, PIN_HT1380_IO, PIN_HT1380_SCLK);

unsigned long timestamp;

void setup(void)
{
  pinMode(PIN_LED, OUTPUT);
  mat.setBrightness(10); // 0 to 15

  rtc.init();

  timestamp = millis();

  Serial.begin(9600);
}

void test_mat(void);
void show_time(int, int);
void set_rtc(uint8_t, uint8_t, uint8_t,
    uint8_t, uint8_t, uint8_t, uint8_t, uint8_t);

void loop(void)
{
  //test_mat();
#if 0
  // update panel in every 1 min
  if (millis() - timestamp > 1000 * 60) {
    rtc.readBurst();
    show_time(rtc.getHour(), rtc.getMin());
  }
#endif
  // #C for clear panel
  // #L<R><C> for turn on a LED in row R and column C
  // #R<h><m><s><Y><M><D><d><wp> for set the RTC
  if (Serial.available() > 1 && '#' == Serial.read()) {
    char func = Serial.read();
    int hour = 0;
    int minute = 0;
    delay(10); // wait enough for following chars
    if (func == 'P') {
      test_mat();
    } else if (func == 'T') {
      hour = 10 * (Serial.read() - '0');
      hour += (Serial.read() - '0');
      minute = 10 * (Serial.read() - '0');
      minute += (Serial.read() - '0');
      show_time(hour, minute);
    } else if (func == 'C') {
      CLEAR_PANEL();
    } else if (func == 'L') {
      P_ON(Serial.read() - '0', Serial.read() - '0');
    } else if (func == 'R') {
      set_rtc(Serial.read(), Serial.read(), Serial.read(),
          Serial.read(), Serial.read(), Serial.read(),
          Serial.read(), Serial.read());
    }
  }

  delay(1000); // sleep 1 sec
  timestamp = millis();
}

void set_rtc(uint8_t h, uint8_t m, uint8_t s,
    uint8_t year, uint8_t month, uint8_t date,
    uint8_t day, uint8_t wp)
{
  // set current time to RTC instance
  rtc.setHour(h);
  rtc.setMin(m);
  rtc.setSec(s);
  rtc.setYear(year);
  rtc.setMonth(month);
  rtc.setDate(date);
  rtc.setDay(day);
  rtc.setWP(wp);

  // write the time to HT1380
  rtc.writeBurst();
}

void show_time(int h, int m)
{
  int m_10 = m / 10;
  int m_1 = m % 10;

  switch (m_1) {
    case 1: case 2: case 3:
      m_1 = 0; break;
    case 4: case 5: case 6:
      m_1 = 5; break;
    case 7: case 8: case 9:
      m_1 = 0; m_10 += 1; break;
  }

  if (m_10 >= 6) {
    m_10 = 0;
    h += 1;
  }

  if (h > 12) h -= 12;

  switch(h) {
    case 0: case 12:
      P_ON(0, 0); P_ON(1, 0); P_ON(2, 4); break;
    case 1: P_ON(0, 1); P_ON(2, 4); break;
    case 2: P_ON(1, 0); P_ON(2, 4); break;
    case 3: P_ON(0, 3); P_ON(2, 4); break;
    case 4: P_ON(0, 4); P_ON(2, 4); break;
    case 5: P_ON(0, 2); P_ON(1, 2); P_ON(2, 4); break;
    case 6: P_ON(1, 1); P_ON(1, 2); P_ON(2, 4); break;
    case 7: P_ON(1, 3); P_ON(1, 4); P_ON(2, 4); break;
    case 8: P_ON(2, 0); P_ON(2, 1); P_ON(2, 4); break;
    case 9: P_ON(2, 2); P_ON(2, 3); P_ON(2, 4); break;
    case 10: P_ON(0, 0); P_ON(2, 4); break;
    case 11: P_ON(0, 0); P_ON(0, 1); P_ON(2, 4); break;
  }

  switch (m_10) {
    case 1: P_ON(3, 4); P_ON(4, 4); break;
    case 2: P_ON(3, 2); P_ON(4, 2); P_ON(4, 4); break;
    case 3: P_ON(3, 3); P_ON(3, 4); P_ON(4, 4); break;
    case 4: P_ON(4, 0); P_ON(4, 2); P_ON(4, 4); break;
    case 5: P_ON(4, 1); P_ON(4, 2); P_ON(4, 4); break;
  }

  if (m_1 == 5)
    P_ON(4, 3);
}

void test_mat(void)
{
  // blink all leds
  for(int i = 0; i < 5; i++) {
    for(int j = 0; j < 5; j++) {
      P_ON(i, j);
    }
  }
  delay(100); mat.clear(); delay(100); 

  // row rotate
  for(int i = 0; i < 5; i++) {
    for(int j = 0; j < 5; j++) {
      P_ON(i, j);
    }
    delay(100); mat.clear();
  }

  // column rotate
  for(int i = 0; i < 5; i++) {
    for(int j = 0; j < 5; j++) {
      P_ON(j, i);
    }
    delay(100); mat.clear();
  }
}

/* vim: set sw=2 et: */
