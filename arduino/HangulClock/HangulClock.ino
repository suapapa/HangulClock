// MAX7219, matrix driver
#include <LedControl.h>

#define PIN_MAX7219_DATA 5//16 //4
#define PIN_MAX7219_CLK  6//15 //3
#define PIN_MAX7219_LOAD 7//14 //2
LedControl lc = LedControl(PIN_MAX7219_DATA,
                           PIN_MAX7219_CLK,
                           PIN_MAX7219_LOAD,
                           1);



byte panel[8] = {0, };

// Fix shuffled row and column idx.
#define R0 4
#define R1 6
#define R2 2
#define R3 3
#define R4 7
#define C0 6
#define C1 1
#define C2 5
#define C3 0
#define C4 4
byte r_table[5] = {R0, R1, R2, R3, R4};
byte c_table[5] = {C0, C1, C2, C3, C4};

#define _P_ON(_R, _C) panel[_R] |= (1 << _C)
#define P_ON(_R, _C) _P_ON(R##_R, C##_C)
#define REFRESH_PANEL() \
  for (int i = 0; i < 8; i++) \
    lc.setRow(0, i, panel[i])

#define CLEAN_PANEL() \
  lc.clearDisplay(0); \
  memset(panel, 0x00, 8)

#define USE_DS1302_RTC
#ifdef USE_DS1302_RTC
#include <DS1302.h>
const int kCePin   = 8;  // Chip Enable
const int kIoPin   = 9;  // Input/Output
const int kSclkPin = 10;  // Serial Clock

DS1302 rtc(kCePin, kIoPin, kSclkPin);
#endif

uint8_t curr_h, curr_m, curr_s;
void set_rtc(uint8_t, uint8_t, uint8_t);
void get_rtc(void);

unsigned long last_millis;
void show_time(int, int);
#define SHOW_CURR_TIME() show_time(curr_h, curr_m)


#define PIN_LED 13

void demo(void);
void splash(void);
void init_ip(void)
{
    Serial.println("Initing matrix driver...");
    lc.shutdown(0, false);
    lc.setIntensity(0, 15);
    lc.clearDisplay(0);

#ifdef USE_DS1302_RTC
    Serial.println("Initing RTC...");
    rtc.writeProtect(false);
    rtc.halt(false);

    // Make a new time object to set the date and time.
    // Sunday, September 22, 2013 at 01:38:50.
    Time t(2013, 9, 22, 1, 38, 50, Time::kSunday);

    // Set the time and date on the chip.
    rtc.time(t);
#endif
}

void setup(void)
{
    pinMode(PIN_LED, OUTPUT);
    digitalWrite(PIN_LED, HIGH);

    Serial.begin(9600);
    init_ip();
    splash();
    last_millis = millis();

    get_rtc();
    SHOW_CURR_TIME();

    digitalWrite(PIN_LED, LOW);
    Serial.println("HELLO");
}

bool tick_tock = LOW;
void loop(void)
{
    unsigned long curr_millis = millis();

    // update panel in every 1 min
    if ((curr_millis - last_millis) >= 5 * 1000) {
        get_rtc();
        SHOW_CURR_TIME();
        last_millis = curr_millis;
    }

    // Serial commands
    // #D for Demo
    // #L<r><c> for turn on a LED in row r and column c
    // #S<hh><mm><ss> set time
    // #G get time
    if (Serial.available() > 1 && '#' == Serial.read()) {
        char func = Serial.read();
        int hour = 0;
        int minute = 0;
        int sec = 0;
        delay(10); // wait enough for following chars
        if (func == 'D') {          // Demo
            demo();
        } else if (func == 'G') {   // Get time
            get_rtc();
            Serial.print((int)(curr_h));
            Serial.print(":");
            Serial.print((int)(curr_m));
            Serial.print(":");
            Serial.print((int)(curr_s));
        } else if (func == 'S') {   // Set time
            hour = 10 * (Serial.read() - '0');
            hour += (Serial.read() - '0');
            minute = 10 * (Serial.read() - '0');
            minute += (Serial.read() - '0');
            sec = 10 * (Serial.read() - '0');
            sec += (Serial.read() - '0');
            set_rtc((uint8_t)hour, (uint8_t)minute, (uint8_t)sec);
            get_rtc();
            SHOW_CURR_TIME();
            last_millis = millis();
        } else if (func == 'L') {
            byte r = Serial.read() - '0';
            byte c = Serial.read() - '0';

            CLEAN_PANEL();
            _P_ON(r_table[r], c_table[c]);
            REFRESH_PANEL();
        }
        Serial.println("OK");
    }

    delay(100); // sleep 100ms
}

int last_shown_h = -1;
int last_shown_m = -1;
void show_time(int h, int m)
{
#if 0
    Serial.print("show_time: ");
    Serial.print(h);
    Serial.print(":");
    Serial.println(m);
#endif

    if (h == last_shown_h && m == last_shown_m)
        return;

    // update current time from RTC in every hour
    if (h != last_shown_h)
        get_rtc();

    last_shown_h = h;
    last_shown_m = m;

    int m_10 = m / 10;
    int m_1 = m % 10;

    switch (m_1) {
    case 1 ... 3:
        m_1 = 0;
        break;
    case 4 ... 6:
        m_1 = 5;
        break;
    case 7 ... 9:
        m_1 = 0;
        m_10 += 1;
        break;
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
    switch (h) {
    case 0:
    case 12:
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

    if (m == 0) {
        REFRESH_PANEL();
        return;
    }

    switch (m_10) {
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

    if (m_1 == 5) {
        P_ON(4, 3); P_ON(4, 4);
    }

    REFRESH_PANEL();
}

void splash(void)
{
    for (int r = 0; r < 5; r++) {
        for (int c = 0; c < 5; c++) {
            CLEAN_PANEL();
            _P_ON(r_table[r], c_table[c]);
            REFRESH_PANEL();
            delay(50);
        }
    }
    CLEAN_PANEL();
}

void demo(void)
{
    for (int h = 0; h < 24; h++) {
        for (int m = 0; m < 60; m += 5) {
            digitalWrite(PIN_LED, HIGH);
            show_time(h, m);
            digitalWrite(PIN_LED, LOW);
            delay(500); // 1sec
        }
    }
    CLEAN_PANEL();
}

#ifdef USE_DS1302_RTC
//uint8_t curr_h, curr_m, curr_s;
void set_rtc(uint8_t h, uint8_t m, uint8_t s)
{
    rtc.hour(h);
    rtc.minutes(m);
    rtc.seconds(s);

    curr_h = h;
    curr_m = m;
    curr_s = s;
}

void get_rtc(void)
{
    curr_h = rtc.hour();
    curr_m = rtc.minutes();
    curr_s = rtc.seconds();
}
#endif

/* vim: set sw=4 et: */
