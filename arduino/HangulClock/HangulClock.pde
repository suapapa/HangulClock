#include <Sprite.h>
#include <Matrix.h>

#define PIN_LED 13

#define PIN_MAX7219_DATA 12
#define PIN_MAX7219_CLK  11
#define PIN_MAX7219_LOAD 10
Matrix mat = Matrix(PIN_MAX7219_DATA, PIN_MAX7219_CLK, PIN_MAX7219_LOAD);

int col[5] = {3, 1, 5, 4, 0};
int row[5] = {5, 1, 6, 2, 7};

void setup(void)
{
  pinMode(PIN_LED, OUTPUT);    
}

void test_mat(void);

void loop(void)
{
  test_mat();
}

void test_mat(void)
{
  // blink all leds
  for(int i = 0; i < 5; i++) {
    for(int j = 0; j < 5; j++) {
      mat.write(row[i], col[j], HIGH);
    }
  }
  delay(1000);
  mat.clear();
  delay(1000);

  // row rotate
  for(int i = 0; i < 5; i++) {
    for(int j = 0; j < 5; j++) {
      mat.write(row[i], col[j], HIGH);
    }
    delay(1000);
    mat.clear();
  }

  // column rotate
  for(int i = 0; i < 5; i++) {
    for(int j = 0; j < 5; j++) {
      mat.write(row[j], col[i], HIGH);
    }
    delay(1000);
    mat.clear();
  }
}

/* vim: set sw=2 et: */
