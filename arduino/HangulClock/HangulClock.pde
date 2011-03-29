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

void loop(void)
{
  digitalWrite(PIN_LED, LOW);
  mat.clear();
  delay(1000);
  for(int i = 0; i < 5; i++) {
    for(int j = 0; j < 5; j++) {
      mat.write(row[i], col[j], HIGH);
    }
  }
  digitalWrite(PIN_LED, HIGH);
  delay(1000);
}

/* vim: set sw=2 et: */
