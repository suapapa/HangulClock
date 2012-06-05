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


#ifdef USE_HT1380_RTC
// HT1380, RTC
#include <HT1380.h>
//HT1380 rtc = HT1380(7, 6, 5);
HT1380 rtc = HT1380(19, 18, 17);
#else
unsigned long last_rtc_millis;
void update_dummy_rtc(void);
#endif


uint8_t curr_h, curr_m, curr_s;
void set_rtc(uint8_t, uint8_t, uint8_t);
void get_rtc(void);

unsigned long last_millis;
unsigned long last_demo_millis;

void show_time(int, int);
#define SHOW_CURR_TIME() show_time(curr_h, curr_m)


#define PIN_LED 13

void demo(void);
void splash(void);
void init_ips(void)
{
    Serial.println("Initing matrix driver...");
    lc.shutdown(0, false);
    lc.setIntensity(0, 15);
    lc.clearDisplay(0);

#ifdef USE_HT1380_RTC
    Serial.println("Initing RTC...");
    rtc.init();
#endif
}

void setup(void)
{
    pinMode(PIN_LED, OUTPUT);
    digitalWrite(PIN_LED, HIGH);

    Serial.begin(9600);
    init_ips();
    splash();
    last_millis = millis();
    last_demo_millis = millis();
#ifndef USE_HT1380_RTC
    set_rtc(10, 5, 30);
#endif
    get_rtc();
    SHOW_CURR_TIME();

    digitalWrite(PIN_LED, LOW);
    Serial.println("HELLO");
}

bool tick_tock = LOW;
void loop(void)
{
    unsigned long curr_millis = millis();


#ifndef USE_HT1380_RTC
    if ((curr_millis - last_rtc_millis) >= 1000) {
        update_dummy_rtc();
        last_rtc_millis = curr_millis;
        tick_tock = (tick_tock == HIGH ? LOW : HIGH);
        digitalWrite(PIN_LED, tick_tock);
    }
#endif

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
            byte demo_no = Serial.read() - '0';
            switch(demo_no) {
              case 1:
                demo();
                break;
              default:
                splash();
                break;
            }
            SHOW_CURR_TIME();
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
    if (h == last_shown_h && m == last_shown_m)
        return;

#if 0
    Serial.print("show_time: ");
    Serial.print(h);
    Serial.print(":");
    Serial.println(m);
    splash();
#endif

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

#ifdef USE_HT1380_RTC
void set_rtc(uint8_t h, uint8_t m, uint8_t s)
{
    rtc.setHour(h);
    rtc.setMin(m);
    rtc.setSec(s);
#if 0
    rtc.setYear(8);
    rtc.setMonth(8);
    rtc.setDate(19);
    rtc.setDay(3);
    rtc.setWP(1);
    delay(1000);
#endif
    rtc.writeBurst();
    delay(1000);
}

void get_rtc(void)
{
    rtc.readBurst();
    curr_h = rtc.getHour();
    curr_m = rtc.getMin();
    curr_s = rtc.getSec();
}
#else
void set_rtc(uint8_t h, uint8_t m, uint8_t s)
{
    curr_h = h;
    curr_m = m;
    curr_s = s;
    last_rtc_millis = millis();
}

void get_rtc(void)
{

}

void update_dummy_rtc(void)
{
    curr_s += 1;
    if (curr_s >= 60) {
        curr_s = 0; curr_m += 1;
    }
    if (curr_m >= 60) {
        curr_m = 0; curr_h += 1;
    }
    if (curr_h >= 24) {
        curr_h = 0;
    }
}

#endif

/* vim: set sw=2 et: */
