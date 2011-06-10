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

void setup(void)
{
  pinMode(PIN_LED, OUTPUT);
  mat.setBrightnexx(10) // 0 to 15
}

void test_mat(void);

void loop(void)
{
  test_mat();
}

void m_on(int h, int m)
{
  switch(h) {
    case 1: break;
    case 2: break;
    case 3: break;
    case 4: break;
    case 5: break;
    case 6: break;
    case 7: break;
    case 8: break;
    case 9: break;
    case 12: break;
    case 13: break;
    case 14: break;
    case 15: break;
    case 16: break;
    case 17: break;
    case 18: break;
    case 19: break;
    case 20: break;
    case 21: break;
    case 22: break;
    case 23: break;
    case 24: break;
  }

  switch (m) {
    case 0: break;
    case 5: break;
    case 10: break;
    case 15: break;
    case 20: break;
    case 25: break;
    case 30: break;
    case 35: break;
    case 40: break;
    case 45: break;
    case 50: break;
    case 55: break;
    case 60: break;
  }

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
