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

#define ROTATE_PANEL_90 1
#if ROTATE_PANEL_90
#define M_ON(M, C, R) _M_ON(M, R, C)
#define M_OFF(M, C, R) _M_OFF(M, R, C)
#else
#define M_ON(M, R, C) _M_ON(M, R, C)
#define M_OFF(M, R, C) _M_OFF(M, R, C)
#endif

#define P_ON(R, C) M_ON(mat, R, C)
#define P_OFF(R, C) M_OFF(mat, R, C)

#define CLEAN_PANEL() \
  mat.clear()

#include <HT1380.h>

#define PIN_HT1380_SCLK 7
#define PIN_HT1380_IO 6
#define PIN_HT1380_REST 5
HT1380 rtc = HT1380(PIN_HT1380_REST, PIN_HT1380_IO, PIN_HT1380_SCLK);
void rtc_set_time(uint8_t, uint8_t);
void rtc_get_time(uint8_t *, uint8_t *);

void demo(void);
void test_mat(void);

void show_time(int, int);

unsigned long timestamp;
void setup(void)
{
  pinMode(PIN_LED, OUTPUT);
  mat.setBrightness(10); // 0 to 15
  //demo();
  test_mat();

  rtc.init();
  rtc_set_time(3, 15);

  timestamp = millis() - (1000 * 61);

  Serial.begin(9600);
}

void loop(void)
{
#if 1
  // update panel in every 1 min
  if (millis() - timestamp >= 1000 * 60) {
    uint8_t h, m;
    rtc_get_time(&h, &m);
    show_time(h, m);
    timestamp = millis();
  }
#endif

  // Serial commands
  // #D for Demo
  // #L<r><c> for turn on a LED in row r and column c
  // #T<hh><mm> show given time to panel
  // #S<hh><mm> set time
  // #G<hh><mm> get time
  if (Serial.available() > 1 && '#' == Serial.read()) {
    char func = Serial.read();
    int hour = 0;
    int minute = 0;
    delay(10); // wait enough for following chars
    if (func == 'D') {          // Demo
      test_mat();
      demo();
      Serial.println("OK");
    } else if (func == 'T') {   // Show time
      hour = 10 * (Serial.read() - '0');
      hour += (Serial.read() - '0');
      minute = 10 * (Serial.read() - '0');
      minute += (Serial.read() - '0');
      show_time(hour, minute);
      Serial.println("OK");
    } else if (func == 'C') {   // Clean panel
      CLEAN_PANEL();
      Serial.println("OK");
    } else if (func == 'L') {   // LED on at
      P_ON(Serial.read() - '0', Serial.read() - '0');
      Serial.println("OK");
    } else if (func == 'G') {   // Get time
      uint8_t h, m;
      rtc_get_time(&h, &m);
      Serial.println("OK");
    } else if (func == 'S') {   // Set time
      hour = 10 * (Serial.read() - '0');
      hour += (Serial.read() - '0');
      minute = 10 * (Serial.read() - '0');
      minute += (Serial.read() - '0');
      rtc_set_time(hour, minute);
      Serial.println("OK");
    }
  }

  delay(1000); // sleep 1 sec
}


void rtc_set_time(uint8_t h, uint8_t m)
{
  rtc.setHour(h);
  rtc.setMin(m);
  rtc.writeBurst();
}

void rtc_get_time(uint8_t *h, uint8_t *m)
{
  rtc.readBurst();
  *h = rtc.getHour();
  *m = rtc.getMin();

  Serial.print((int)(*h));
  Serial.print(":");
  Serial.print((int)(*m));
  Serial.print(":");
  Serial.println((int)rtc.getSec());
}

void show_time(int h, int m)
{
  if (h > 24) return;
  if (m > 60) return;

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

  CLEAN_PANEL();

  if ((h == 0 || h == 24) && (m_10 + m_1) == 0) {
    P_ON(3, 0); P_ON(3, 1);
    return;
  }

  if (h == 12 && (m_10 + m_1) == 0) {
    P_ON(3, 1); P_ON(4, 1);
    return;
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

void demo(void)
{
  for(int h = 0; h < 24; h++) {
    for(int m = 0; m < 60; m+=5) {
      digitalWrite(PIN_LED, HIGH);
      show_time(h, m);
      digitalWrite(PIN_LED, LOW);
      delay(500); // 1sec
    }
  }
}


/* vim: set sw=2 et: */
