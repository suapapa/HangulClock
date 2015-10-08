// MAX7219, matrix driver
#include <LedControl.h>
#include <Button.h>

#define USE_DS1307_RTC
#define SET_TIME_BTN
#define ANI_MODE_KEY

#define PIN_MAX7219_DATA 5//16 //4
#define PIN_MAX7219_CLK  6//15 //3
#define PIN_MAX7219_LOAD 7//14 //2
LedControl lc = LedControl(PIN_MAX7219_DATA, PIN_MAX7219_CLK, PIN_MAX7219_LOAD, 1);

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

#ifdef USE_DS1307_RTC
// Date and time functions using a DS1307 RTC connected via I2C and Wire lib
#include <Wire.h>
#include "RTClib.h"
RTC_DS1307 RTC;
#else
void update_dummy_rtc(void);
#endif

unsigned long last_rtc_millis;

uint8_t curr_h, curr_m, curr_s;
void set_rtc(uint8_t, uint8_t, uint8_t);
void get_rtc(void);

unsigned long last_millis;
void show_time(int, int);
#define SHOW_CURR_TIME() show_time(curr_h, curr_m)

#define PIN_LED 13

int animation_mode = false;
int sp=0;
#define MAX_ROW  5
#define MAX_COL  5
#define MAX_BUFF_SIZE 40 //MAX_ROW*MAX_COL

int photo_no = 0;

void Animation(byte *buff, int index, int buff_sz)
{
  //byte *buff;
  int i, j;

  CLEAN_PANEL();
  for(i=0; i<MAX_ROW; i++){
    for(j=0; j<MAX_COL; j++) {
      if(buff[(i+index)%buff_sz] & (0x01 << j))
        _P_ON(r_table[i], c_table[(MAX_COL-1)-j]);
    }
  }
  REFRESH_PANEL();
}

void PictureMode()
{
  int i, j;

  CLEAN_PANEL();
  for(i=0; i<MAX_ROW; i++){
    for(j=0; j<MAX_COL; j++) {
      _P_ON(r_table[i], c_table[j]);
    }
  }
  REFRESH_PANEL();
}

#if 0
//=======================================================
// Func. Description
// Animate screen data to move horizontal direction
// Parameter :
//       - *buff : to display data buffer pointer
//       - index : buffer position to read
//       - buff_sz : size of buffer to read
//       - bit_pos : position of bit to display byte in horizontal mode.
//                   if animation mode is virtical direction, it must set to ZERO.
//                   valide range of value is 0 to MAX_COL-1
//
// Return value : RxBuff read position increament flag
//               true - increase read rx buff. position (sp++)
//               false - Hold sp
//=======================================================
boolean AnimationHorizon(byte *buff, int index, int buff_sz, int bit_pos)
{
  //byte *buff;
  int i, j, shift, next;

  CLEAN_PANEL();
  for(i=0; i<MAX_ROW; i++){
    for(j=0; j<MAX_COL; j++) {
      next = (j+bit_pos >= MAX_COL) ? 5 : 0;
      shift = (next==5) ? ((j+bit_pos)-MAX_COL) : j;
      if(buff[(i+index+next)%buff_sz] & (0x01 << (MAX_COL-1)-shift))
        _P_ON(r_table[i], c_table[j]);
    }
  }
  REFRESH_PANEL();
}
#endif

#define  ANI_CLEAR   0
#define  ANI_NORMAL  1 // Default Animation
#define  ANI_USER_V  2 // User Image Animation (vertical direction)
#define  ANI_USER_H  3 // User Image Animation (Horizontal direction)
#define  ANI_USER_F  4 // User Image Animation (Flash mode)
#define  ANI_INFO    5 // Weather Info Display
#define  ANI_DEMO    6 // Time Demo
#define  ANI_PICTURE 7 // Picture Mode

boolean picture_mode = false;

boolean AniModeSet(int mode)
{
  if(mode != ANI_PICTURE)
    picture_mode = false;
  animation_mode = mode;
  sp=0; // display position reset.
  Serial.print("AniMode = ");
  Serial.println(mode);
  return((mode==ANI_CLEAR) ? false:true);
}

#ifdef SET_TIME_BTN
#define PIN_BTN_UP 9
#define PIN_BTN_DN 8
#define LONGPRESS_CNT 20 //10

int btnUp_pressed_cnt = 0;
int btnDn_pressed_cnt = 0;

Button btnUp = Button(PIN_BTN_UP,PULLUP);
Button btnDn = Button(PIN_BTN_DN,PULLUP);

boolean flgReqSysTime = 0;

uint8_t Inc5Min(uint8_t time)
{
  if((time==2) || (time==3))
    return 3;
  else if((time==7) || (time==8))
    return 7;
  else
    return 5;
}

boolean button_chk(void)
{
  boolean change = false;

  // Button Up check
  if(btnUp.uniquePress()){
    btnUp_pressed_cnt = 0;

    if(animation_mode) {
      AniModeSet(ANI_CLEAR); // off animation_mode
      Serial.println("animode clear");
      return(true);
    }
    get_rtc();
    // Inc. Hour
    curr_h++;
    if(curr_h >= 24) curr_h = curr_h%24;
    change = true;
    //Serial.print("Change hour to ");
    //Serial.println(curr_h);
  }else if(btnUp.wasPressed()){  // Long Key check
    btnUp_pressed_cnt++;
    if(btnUp_pressed_cnt > LONGPRESS_CNT){
      btnUp_pressed_cnt = 0;
      // Request system time to PC
      //flgReqSysTime = 1;
      //Serial.println("Req. System Time Flag set");
      //req_sys_time();
    }
  }else if(btnUp.stateChanged()){
    //Serial.println("BtnUp State Change");
  }

  // Button down check
  if(btnDn.uniquePress()){
    //Serial.println("Button Down Click");
    btnDn_pressed_cnt = 0;

    get_rtc();
    // inc. Min.
    curr_m += Inc5Min(curr_m);
    if(curr_m >= 60) curr_m = curr_m%60;
    change = true;
    //Serial.print("Change Minutes to ");
    //Serial.println(curr_m);
  }else if(btnDn.wasPressed()){
    //Serial.println("BtnDn WAS pressed");
    btnDn_pressed_cnt++;
    if(btnDn_pressed_cnt > LONGPRESS_CNT){
      //Serial.println("BtnDn Long Key");
#ifdef ANI_MODE_KEY
      if(!animation_mode) {
        //Serial.println("Set Ani_Normal2");
        //AniModeSet(ANI_NORMAL);
        AniModeSet(ANI_PICTURE);
        photo_no = 0;
        btnDn_pressed_cnt = 0;
        //Serial.println("animode nomal");
        // off animation mode
        return(true);
      }else {
        AniModeSet(ANI_CLEAR); // if animation mode is set, will be exit animation_mode.
        //Serial.println("exit animation mode");
      }
#else
      curr_m++;
      if(curr_m >= 60) curr_m = curr_m%60;
      change = true;
      //Serial.print("Change Minutes to ");
      //Serial.println(curr_m);
#endif
    }
  }else if(btnDn.stateChanged()){
    //Serial.println("BtnDn State Change");
  }

  return(change);
}
#endif


void demo(void);
void splash(void);
void init_ips(void)
{
  Serial.println("Initing matrix driver...");
  lc.shutdown(0, false);
  lc.setIntensity(0, 7);
  lc.clearDisplay(0);

#ifdef USE_DS1307_RTC
  Serial.println("Initing RTC...");
#endif
}

#ifdef USE_DS1307_RTC
#define DS1307_I2C_ADDRESS 0x68

// Convert normal decimal numbers to binary coded decimal
byte decToBcd(byte val){
  return ( (val/10*16) + (val%10) );
}

// Convert binary coded decimal to normal decimal numbers
byte bcdToDec(byte val){
  return ( (val/16*10) + (val%16) );
}

// 1) Sets the date and time on the ds1307
// 2) Starts the clock
// 3) Sets hour mode to 24 hour clock

// Assumes you're passing in valid numbers

void setDateDs1307(byte second,        // 0-59
    byte minute,        // 0-59
    byte hour,          // 1-23
    byte dayOfWeek,     // 1-7
    byte dayOfMonth,    // 1-28/29/30/31
    byte month,         // 1-12
    byte year)          // 0-99
{
  Wire.beginTransmission(DS1307_I2C_ADDRESS);
  Wire.write(0);
  Wire.write(decToBcd(second));    // 0 to bit 7 starts the clock
  Wire.write(decToBcd(minute));
  Wire.write(decToBcd(hour));
  Wire.write(decToBcd(dayOfWeek));
  Wire.write(decToBcd(dayOfMonth));
  Wire.write(decToBcd(month));
  Wire.write(decToBcd(year));
  Wire.write(00010000); // sends 0x10 (hex) 00010000 (binary) to control register - turns on square wave
  Wire.endTransmission();
}

// Gets the date and time from the ds1307
void getDateDs1307(byte *second, byte *minute, byte *hour, byte *dayOfWeek, byte *dayOfMonth, byte *month, byte *year){
  // Reset the register pointer
  Wire.beginTransmission(DS1307_I2C_ADDRESS);
  Wire.write(0);
  Wire.endTransmission();

  Wire.requestFrom(DS1307_I2C_ADDRESS, 7);

  // A few of these need masks because certain bits are control bits
  *second     = bcdToDec(Wire.read() & 0x7f);
  *minute     = bcdToDec(Wire.read());
  *hour       = bcdToDec(Wire.read() & 0x3f);  // Need to change this if 12 hour am/pm
  *dayOfWeek  = bcdToDec(Wire.read());
  *dayOfMonth = bcdToDec(Wire.read());
  *month      = bcdToDec(Wire.read());
  *year       = bcdToDec(Wire.read());
}

#endif

void setup(void)
{
  pinMode(PIN_LED, OUTPUT);
  digitalWrite(PIN_LED, HIGH);

#ifdef USE_DS1307_RTC
  Serial.begin(57600);
  Wire.begin();
  RTC.begin();

  if (! RTC.isrunning()) {
    Serial.println("RTC is NOT running!");
    // following line sets the RTC to the date & time this sketch was compiled
    RTC.adjust(DateTime(__DATE__, __TIME__));
  }
#else
  Serial.begin(9600);
#endif
  init_ips();
  splash();
  last_millis = millis();
#ifndef USE_DS1307_RTC
  set_rtc(10, 32, 45);
#endif

  get_rtc();
  SHOW_CURR_TIME();

  digitalWrite(PIN_LED, LOW);
  Serial.println("HELLO");
  Serial.flush();

  // For Test
  //Serial.println(RAMEND);
  Serial.println(SERIAL_RX_BUFFER_SIZE);
}

void req_sys_time(void)
{
  // For test read time(HH:MM:SS) from PC
  Serial.println("#S|T|[]#");
}
void req_sys_date(void)
{
  // For test read date(YYYY-MM-DD) from PC
  Serial.println("#S|D|[]#");
}

#define SYS_TIME_LEN  8 /* 11 */
#define SYS_DATE_LEN  10

void set_sys_time()
{
  char Buff[30];
  char *receiveSerial;      // received data buffer pointer
  int i,foundPos;
  int receiveLeng;

  //delay(100);  // wait for rx data from PC
  receiveSerial = &Buff[0];
  receiveLeng = Serial.available();
  Serial.print("receiveLeng = ");
  Serial.println(receiveLeng);

  if (receiveLeng >= SYS_TIME_LEN) {

    Serial.readBytes(receiveSerial, receiveLeng);

    for(i=0; i < receiveLeng; i++) {
      //Serial.print(receiveSerial[i]);
      if(receiveSerial[i]==':') {
        foundPos = i;
        break;
      }
    }

    // set time
    if((receiveSerial[foundPos-2] != '1') && (receiveSerial[foundPos-2] != '2'))
      curr_h = receiveSerial[foundPos-1] - '0';
    else
      curr_h = (receiveSerial[foundPos-2] - '0')*10 + (receiveSerial[foundPos-1] - '0');

    curr_m = (receiveSerial[foundPos+1] - '0')*10 + (receiveSerial[foundPos+2] - '0');
    curr_s = (receiveSerial[foundPos+4] - '0')*10 + (receiveSerial[foundPos+5] - '0');
#if 0
    Serial.print(curr_h);
    Serial.print(":");
    Serial.print(curr_m);
    Serial.print(":");
    Serial.println(curr_s);
#endif

    set_rtc(curr_h, curr_m, curr_s);

    Serial.flush();
  }
}

/*
   { 0x06, 0x02, 0x0E, 0x0A, 0x0F, 0x00,  // a
   0x08, 0x08, 0x0E, 0x0A, 0x0E, 0x00,  // b
   0x00, 0x06, 0x08, 0x08, 0x06, 0x00,  // c
   0x02, 0x02, 0x0E, 0x0A, 0x0E, 0x00,  // d
   0x00, 0x0E, 0x0A, 0x0C, 0x0E, 0x00,  // e
   0x06, 0x08, 0x0C, 0x08, 0x08, 0x00,  // f
   0x0C, 0x0A, 0x0E, 0x02, 0x0C, 0x00,  // g
   0x08, 0x08, 0x0C, 0x0A, 0x0A, 0x00,  // h
   0x04, 0x00, 0x04, 0x04, 0x04, 0x00,  // i
   0x04, 0x00, 0x04, 0x04, 0x0C, 0x00,  // j
   0x08, 0x08, 0x0E, 0x0C, 0x0A, 0x00,  // k
   0x04, 0x04, 0x04, 0x04, 0x06, 0x00,  // l
   0x00, 0x0E, 0x0E, 0x0E, 0x0A, 0x00,  // m
   0x00, 0x0E, 0x0A, 0x0A, 0x0A, 0x00,  // n
   0x00, 0x04, 0x0A, 0x0A, 0x04, 0x00,  // o
   0x00, 0x0C, 0x0A, 0x0C, 0x08, 0x08,  // p
   0x00, 0x06, 0x0A, 0x06, 0x02, 0x02,  // q
   0x00, 0x0C, 0x0A, 0x0C, 0x0A, 0x00,  // r
   0x00, 0x0E, 0x0C, 0x02, 0x0E, 0x00,  // s
   0x00, 0x0E, 0x04, 0x04, 0x04, 0x00,  // t
   0x00, 0x0A, 0x0A, 0x0A, 0x06, 0x00,  // u
   0x00, 0x0A, 0x0A, 0x0E, 0x04, 0x00,  // v
   0x00, 0x0A, 0x0E, 0x0E, 0x0A, 0x00,  // w
   0x00, 0x0A, 0x0E, 0x04, 0x0A, 0x00,  // x
   0x00, 0x0A, 0x0A, 0x0E, 0x02, 0x0C,  // y
   0x00, 0x0E, 0x02, 0x0C, 0x0E, 0x00,  // z
   }

   { 0x0E, 0x19, 0x1F, 0x19, 0x19, 0x00,  // A
   0x1E, 0x19, 0x1E, 0x19, 0x1E, 0x00,  // B
   0x0E, 0x19, 0x18, 0x19, 0x0E, 0x00,  // C
   0x1E, 0x19, 0x19, 0x19, 0x1E, 0x00,  // D
   0x1F, 0x18, 0x1F, 0x18, 0x1F, 0x00,  // E
   0x1F, 0x18, 0x1E, 0x18, 0x18, 0x00,  // F
   0x0E, 0x18, 0x1B, 0x19, 0x0E, 0x00,  // G
   0x19, 0x19, 0x1F, 0x19, 0x19, 0x00,  // H
   0x0E, 0x04, 0x00, 0x04, 0x0E, 0x00,  // I
   0x0E, 0x04, 0x00, 0x14, 0x0C, 0x00,  // J
   0x19, 0x1A, 0x1C, 0x1E, 0x1B, 0x00,  // K
   0x18, 0x18, 0x18, 0x18, 0x1F, 0x00,  // L
   0x11, 0x1B, 0x19, 0x19, 0x19, 0x00,  // M
   0x11, 0x19, 0x19, 0x1B, 0x19, 0x00,  // N
   0x0E, 0x19, 0x19, 0x19, 0x0E, 0x00,  // O
   0x1E, 0x19, 0x1E, 0x18, 0x18, 0x00,  // P
   0x0E, 0x19, 0x19, 0x1B, 0x0F, 0x00,  // Q
   0x1E, 0x19, 0x19, 0x1E, 0x1B, 0x00,  // R
   0x0F, 0x18, 0x1F, 0x01, 0x1E, 0x00,  // S
   0x1F, 0x04, 0x04, 0x04, 0x04, 0x00,  // T
   0x19, 0x19, 0x19, 0x19, 0x0E, 0x00,  // U
   0x11, 0x19, 0x19, 0x0A, 0x04, 0x00,  // V
   0x11, 0x11, 0x14, 0x1B, 0x0A, 0x00,  // W
   0x11, 0x0A, 0x04, 0x0A, 0x11, 0x00,  // X
   0x11, 0x19, 0x0C, 0x04, 0x04, 0x00,  // Y
   0x1F, 0x03, 0x10, 0x0C, 0x1F, 0x00,  // Z
   }
 */
#if 0
const byte view_buff[] = { 0x0A, 0x15, 0x11, 0x0A, 0x04, // Heart
  0x00, 0x1B, 0x1F, 0x0E, 0x04, // Heart
  0x00, 0x08, 0x08, 0x0E, 0x00, // L
  0x00, 0x04, 0x0A, 0x0A, 0x04, // o
  0x00, 0x0A, 0x0A, 0x0E, 0x04, // v
  0x00, 0x0E, 0x0A, 0x0C, 0x0E, // e
  0x0A, 0x15, 0x11, 0x0A, 0x04, // Heart
  0x00, 0x1B, 0x1F, 0x0E, 0x04, // Heart
};
#else
const byte view_buff[] = { 0x0A, 0x15, 0x11, 0x0A, 0x04, 0x00, // Heart
  0x18, 0x18, 0x18, 0x18, 0x1F, 0x00, // L
  0x0E, 0x19, 0x19, 0x19, 0x0E, 0x00, // O
  0x11, 0x19, 0x1B, 0x0E, 0x04, 0x00, // V
  0x1F, 0x18, 0x1E, 0x18, 0x1F, 0x00, // E
  0x00, 0x1B, 0x1F, 0x0E, 0x04, 0x00, // Heart
};
#endif

const byte photo_buff[] = {
  0x18,0x10,0x00,0x00,0x00,
  0x1C,0x18,0x10,0x00,0x00,
  0x1E,0x1C,0x18,0x10,0x00,
  0x1F,0x1E,0x1C,0x18,0x10,
  0x1F,0x1F,0x1E,0x1C,0x18,
  0x1F,0x1F,0x1F,0x1E,0x1C,
  0x1F,0x1F,0x1F,0x1F,0x1F,
  0x1F,0x1F,0x1F,0x1F,0x1F,

};

const byte photo2_buff[] = {
  0x00,0x00,0x04,0x00,0x00,
  0x00,0x08,0x0C,0x00,0x00,
  0x00,0x0E,0x0E,0x02,0x00,
  0x10,0x1E,0x1E,0x1E,0x00,
  0x1F,0x1F,0x1F,0x1F,0x01,
  0x1F,0x1F,0x1F,0x1F,0x1F,
  0x1F,0x1F,0x1F,0x1F,0x1F,
  0x1F,0x1F,0x1F,0x1F,0x1F,
};

const byte rainy_day[] = {
  0x08,0x08,0x0A,0x02,0x02,
  0x01,0x08,0x08,0x0A,0x02,
  0x11,0x01,0x08,0x08,0x0A,
  0x11,0x11,0x01,0x08,0x08,
  0x14,0x11,0x11,0x01,0x08,
  0x04,0x14,0x11,0x11,0x01,
  0x04,0x04,0x14,0x11,0x11,
  0x00,0x04,0x04,0x14,0x11,
};

const byte haze_day[] = {
  0x00,0x00,0x00,0x02,0x00,
  0x00,0x08,0x02,0x08,0x00,
  0x00,0x00,0x12,0x05,0x10,
  0x10,0x02,0x14,0x01,0x14,
  0x12,0x01,0x0A,0x00,0x15,
  0x14,0x0A,0x11,0x08,0x15,
  0x09,0x14,0x0A,0x00,0x15,
  0x0A,0x15,0x0A,0x15,0x0A,
};

const byte normal_day[] = {
  0x00,0x00,0x04,0x00,0x00,
  0x00,0x0E,0x0A,0x0E,0x00,
  0x1F,0x11,0x15,0x11,0x1F,
  0x00,0x0E,0x0A,0x0E,0x00,
  0x00,0x00,0x04,0x00,0x00,
  0x00,0x0E,0x0A,0x0E,0x00,
  0x1F,0x11,0x15,0x11,0x1F,
  0x00,0x0E,0x0A,0x0E,0x00,
};

const byte cloudy_day[] = {
  0x00,0x1B,0x1E,0x0C,0x00,
  0x01,0x16,0x1C,0x18,0x00,
  0x02,0x0C,0x18,0x10,0x01,
  0x04,0x18,0x10,0x00,0x02,
  0x08,0x11,0x01,0x01,0x04,
  0x10,0x03,0x03,0x02,0x08,
  0x00,0x06,0x07,0x05,0x10,
  0x00,0x0D,0x0F,0x08,0x00,
};

const byte snowy_day[] = {
  0x14,0x02,0x09,0x12,0x04,
  0x09,0x14,0x02,0x09,0x12,
  0x02,0x09,0x14,0x02,0x09,
  0x11,0x02,0x08,0x14,0x02,
  0x04,0x11,0x02,0x08,0x14,
  0x12,0x04,0x11,0x02,0x08,
  0x08,0x12,0x04,0x11,0x02,
  0x04,0x08,0x12,0x04,0x11,
};

byte RxBuff[MAX_BUFF_SIZE+1];
int RxBuffSize = 0;

#define NORMAL     0
#define RAINY_DAY  1
#define HAZE_DAY   2
#define CLOUDY_DAY 3
#define SNOWY_DAY  4
#define MAX_WEATHER_DISP_CNT  2

int Weather = NORMAL;
int WeatherDispCount = 0;
bool tick_tock = LOW;
void loop(void)
{
  int i, j;
  unsigned long curr_millis = millis();
  byte *photo;

#ifndef USE_DS1307_RTC
  if ((curr_millis - last_rtc_millis) >= 1000) {
    update_dummy_rtc();
    last_rtc_millis = curr_millis;
    tick_tock = (tick_tock == HIGH ? LOW : HIGH);
    digitalWrite(PIN_LED, tick_tock);
  }
#endif

#ifdef SET_TIME_BTN
  // button check
  if(button_chk()) {
    //Serial.println("button_chk() return true!!");
    set_rtc(curr_h, curr_m, curr_s);
    get_rtc();
    SHOW_CURR_TIME();
    last_millis = millis();
  }
#endif

  // update panel in every 1 min
  if ((curr_millis - last_millis) >= 30 * 1000) {
    get_rtc();
    SHOW_CURR_TIME();
    last_millis = curr_millis;
  }

  // Set System Time from PC
  if(flgReqSysTime) {
    //Serial.println("Check Req. system time flag");
    delay(100); // wait for rx data from PC
    if (Serial.available() >= SYS_TIME_LEN) {
      flgReqSysTime = 0;
      Serial.println("Get system time");
      set_sys_time();
      get_rtc();
      SHOW_CURR_TIME();
      last_millis = millis();
    }
  }else if(animation_mode) {
    switch(animation_mode) {
      case ANI_INFO :
        //Serial.print("Weather is ");
        //Serial.println(Weather);
        if(Weather == RAINY_DAY){
          Animation((byte *)&rainy_day[0], sp, MAX_BUFF_SIZE);
        }else if(Weather == HAZE_DAY) {
          Animation((byte *)&haze_day[0], sp, MAX_BUFF_SIZE);
        }else if(Weather == CLOUDY_DAY){
          Animation((byte *)&cloudy_day[0], sp, MAX_BUFF_SIZE);
        }else if(Weather == SNOWY_DAY){
          Animation((byte *)&snowy_day[0], sp, MAX_BUFF_SIZE);
        }else{
          Animation((byte *) &normal_day[0], sp, MAX_BUFF_SIZE);
        }
        if(sp < MAX_BUFF_SIZE) sp += 5;
        else {
          sp = 0;
          WeatherDispCount++;
          if(WeatherDispCount > MAX_WEATHER_DISP_CNT) {
            WeatherDispCount = 0;
            //AniModeSet(ANI_CLEAR);
          }
        }
        break;

      case ANI_USER_V :
      case ANI_USER_F :
        //Serial.println("ani. mode = ANI_USER");
        Animation(&RxBuff[0], sp, RxBuffSize);

        if(sp < RxBuffSize) {
          if(animation_mode == ANI_USER_V)
            sp++;
          else if (animation_mode == ANI_USER_F)
            sp += 5;
        }else sp = 0;
        break;

      case ANI_PICTURE :
        if(!picture_mode) {
          picture_mode = true;
          PictureMode();
        }else {
          if(photo_no==0) Animation((byte *)&photo_buff[0], sp, MAX_BUFF_SIZE);
          else if(photo_no==1) Animation((byte *)&photo2_buff[0], sp, MAX_BUFF_SIZE);
          else if(photo_no==2) PictureMode();

          if(sp < MAX_BUFF_SIZE) {
            sp += 5;
          }else {
            sp = 0;
            photo_no++;
            if(photo_no > 5) photo_no = 5;
          }
        }
        break;

      case ANI_USER_H :
        //break;

      case ANI_NORMAL :
        //Serial.println("ani. mode = ANI_NORMAL");

      default :
        Animation((byte *)&view_buff[0], sp, MAX_BUFF_SIZE);
        if(sp < MAX_BUFF_SIZE) sp++;
        else sp = 0;
        break;
    }
    delay(100);
  }

  // Serial commands
  // #D for Demo
  // #L<r><c> for turn on a LED in row r and column c
  // #S<hh><mm><ss> set time
  // #G get time
  // #V Veritical View Mode (animate received data from PC)
  // #H Horizontal View Mode (animate received data from PC)
  // #F Flash View Mode (animate received data from PC)
  // #A Animation Mode (animate default image)
  // #C Time Display Mode ( Exit animation mode )
  // #W Weather Display Mode
  // #P Photo Mode
  if (Serial.available() > 1 && '#' == Serial.read()) {
    char func = Serial.read();
    int hour = 0;
    int minute = 0;
    int sec = 0;
    byte temp;
    delay(10); // wait enough for following chars
    if (func == 'D') {          // Demo
      AniModeSet(ANI_DEMO);
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
    } else if ((func == 'V') || (func == 'H') || (func == 'F')) {
      Serial.print("RxLen = ");
      Serial.println(Serial.available());
      for(i=0; Serial.available() > 0; ) {
        RxBuff[i] = Serial.read();
        if(RxBuff[i]!=',') { // Hex first char.
          temp = RxBuff[i] - '0';
          if(temp > 9) temp = (RxBuff[i] > 'F') ? (RxBuff[i] - 87) : (RxBuff[i] - 55);
          //Serial.print(temp);
          RxBuff[i] = Serial.read();
          if(RxBuff[i] != ','){ // Hex second number is exist.
            temp += (temp !=0 ) ? 15 : 0;
            if(RxBuff[i] > '9') {
              temp += (RxBuff[i] > 'F') ? (RxBuff[i] - 87) : (RxBuff[i] - 55);
            }
            else temp += (RxBuff[i] - '0'); // this is Numeric.
            //Serial.println(temp);
          }
          RxBuff[i] = temp;
#if 0
          Serial.print("RxBuff[");
          Serial.print(i);
          Serial.print("]=");
          Serial.println(temp);
#endif
          i++;
        }
      }
      RxBuff[i] = 0xFE; // Added EOL..
      RxBuffSize = i;
      Serial.print("Received Length = ");
      Serial.println(i);
      //Serial.println(RxBuff[i]);
      if(func == 'V')
        AniModeSet(ANI_USER_V);
      else if (func == 'H')
        AniModeSet(ANI_USER_H);
      else if (func == 'F')
        AniModeSet(ANI_USER_F);
    }else if (func == 'A') {
      Serial.println("Set Ani_Normal");
      AniModeSet(ANI_NORMAL);
    }else if (func == 'C') {
      Serial.println("Exit Animation Mode");
      AniModeSet(ANI_CLEAR);
    }else if (func == 'W') {
      char WCode = Serial.read();
      if(WCode == 'R'){
        // It's Rainy day
        Weather = RAINY_DAY;
        Serial.println("It's Rainy");
      }else if (WCode == 'H') {
        // It's Haze
        Weather = HAZE_DAY;
        Serial.println("It's Haze");
      }else if(WCode == 'C') {
        // It's Cloudy Day
        Weather = CLOUDY_DAY;
        Serial.println("It's Cloudy");
      }else if(WCode == 'S') {
        // It's Cloudy Day
        Weather = SNOWY_DAY;
        Serial.println("It's Snowy");
      }else Weather = NORMAL;
      AniModeSet(ANI_INFO);
    }else if (func == 'P') {
      AniModeSet(ANI_PICTURE);
      photo_no = 0;
    }
    Serial.println(" OK");
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

#if 0
  if (h == last_shown_h && m == last_shown_m)
    return;
#endif

  if(picture_mode) return;

  //if(animation_mode != ANI_CLEAR) return; // Must be check!!!

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
    //Serial.print("m_10=");
    //Serial.println(m_10);
    m_10 = 0;
    h += 1;
  }

  CLEAN_PANEL();

  if ((h == 0 || h == 24) && (m_10 + m_1) == 0) {
    //Serial.println("JaJung...");
    P_ON(3, 0); P_ON(3, 1); // JaJung
    REFRESH_PANEL();
    return;
  }

  if (h == 12 && (m_10 + m_1) == 0) {
    //Serial.println("JungOh...");
    P_ON(3, 1); P_ON(4, 1); // JungOh
    REFRESH_PANEL();
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
      //if(animation_mode != ANI_DEMO) return;
      digitalWrite(PIN_LED, HIGH);
      show_time(h, m);
      digitalWrite(PIN_LED, LOW);
      delay(500); // 1sec
    }
  }
  CLEAN_PANEL();
}

#ifdef USE_DS1307_RTC
byte sec, min, hour, DoWeek, DoMonth, month, year;
void set_rtc(uint8_t h, uint8_t m, uint8_t s)
{
  getDateDs1307(&sec, &min, &hour, &DoWeek, &DoMonth, &month, &year); // 20150608 added by bclee.
#if 0
  Serial.print(year);
  Serial.print(".");
  Serial.print(month);
  Serial.print(".");
  Serial.print(DoMonth);
  Serial.print(". ");
#endif
  setDateDs1307(s, m, h, DoWeek, DoMonth, month, year);
  curr_h = h;
  curr_m = m;
  curr_s = s;
  last_rtc_millis = millis();
}

void get_rtc(void)
{
  DateTime now = RTC.now();

  curr_h = now.hour();
  curr_m = now.minute();
  curr_s = now.second();
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
