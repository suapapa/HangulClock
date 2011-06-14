#include <Sprite.h>
#include <Matrix.h>

#define PIN_LED 13

#define PIN_MAX7219_DATA 12
#define PIN_MAX7219_CLK  11
#define PIN_MAX7219_LOAD 10
Matrix mat = Matrix(PIN_MAX7219_DATA, PIN_MAX7219_CLK, PIN_MAX7219_LOAD);

int col[5] = {3, 1, 5, 4, 0};
int row[5] = {5, 1, 6, 2, 7};
#define M_ON(M, R, C) M.write(row[R], col[C], HIGH)
#define M_OFF(M, R, C) M.write(row[R], col[C], LOW)

Sprite timePanel = Sprite(
  5, 5,
  B00000,
  B00000,
  B00000,
  B00000,
  B00000
);
#define P_ON(R, C) M_ON(timePanel, R, C)
#define P_OFF(R, C) M_OFF(timePanel, R, C)

#define CLEAR_PANEL() \
  for(int i = 0; i < 5; i++) {\
    for(int j = 0; j < 5; j++) {\
      P_OFF(i, j);\
    }\
  }

#include <HT1380.h>

#define PIN_HT1380_REST 7
#define PIN_HT1380_IO 6
#define PIN_HT1380_SCLK 5
HT1380 rtc = HT1380(PIN_HT1380_REST, PIN_HT1380_IO, PIN_HT1380_SCLK);

void setup(void)
{
  pinMode(PIN_LED, OUTPUT);
  mat.setBrightness(10); // 0 to 15

  rtc.init();
}

void test_mat(void);

void loop(void)
{
  test_mat();
}

void m_on(int h, int m)
{
  if (h > 12) h -= 12;
  switch(h) {
    case 0: case 12:
      P_ON(0, 0); P_ON(1, 0); P_ON(2, 4);
      break;
    case 1:
      P_ON(0, 1); P_ON(2, 4);
      break;
    case 2:
      P_ON(1, 0); P_ON(2, 4);
      break;
    case 3:
      P_ON(0, 3); P_ON(2, 4);
      break;
    case 4:
      P_ON(0, 4); P_ON(2, 4);
      break;
    case 5:
      P_ON(0, 2); P_ON(1, 2); P_ON(2, 4);
      break;
    case 6:
      P_ON(1, 1); P_ON(1, 2); P_ON(2, 4);
      break;
    case 7:
      P_ON(1, 3); P_ON(1, 4); P_ON(2, 4);
      break;
    case 8:
      P_ON(2, 0); P_ON(2, 1); P_ON(2, 4);
      break;
    case 9:
      P_ON(2, 2); P_ON(2, 3); P_ON(2, 4);
      break;
    case 10:
      P_ON(0, 0); P_ON(2, 4);
      break;
    case 11:
      P_ON(0, 0); P_ON(0, 1); P_ON(2, 4);
      break;
  }

  switch (m / 10) {
    case 1:
      P_ON(3, 4); P_ON(4, 4);
      break;
    case 2:
      P_ON(3, 2); P_ON(4, 2); P_ON(4, 4);
      break;
    case 3:
      P_ON(3, 3); P_ON(3, 4); P_ON(4, 4);
      break;
    case 4:
      P_ON(4, 0); P_ON(4, 2); P_ON(4, 4);
      break;
    case 5:
      P_ON(4, 1); P_ON(4, 2); P_ON(4, 4);
      break;
  }

  if (m % 10 == 5)
    P_ON(4, 3);
}

void test_mat(void)
{
  // blink all leds
  for(int i = 0; i < 5; i++) {
    for(int j = 0; j < 5; j++) {
      M_ON(mat, i, j);
    }
  }
  delay(1000);
  mat.clear();
  delay(1000);

  // row rotate
  for(int i = 0; i < 5; i++) {
    for(int j = 0; j < 5; j++) {
      M_ON(mat, i, j);
    }
    delay(1000);
    mat.clear();
  }

  // column rotate
  for(int i = 0; i < 5; i++) {
    for(int j = 0; j < 5; j++) {
      M_ON(mat, j, i);
    }
    delay(1000);
    mat.clear();
  }
}

/* vim: set sw=2 et: */
