#include "TinyGPS++.h" // https://github.com/mikalhart/TinyGPSPlus

// GPSに接続したシリアル通信
#define GPS_SERIAL Serial1

TinyGPSPlus gps;
// Serial1のRX,TXピン
const int PIN_GPS_RX = 16;
const int PIN_GPS_TX = 17;
void setup() {
  Serial.begin(19200);
  GpsInit();
}

void loop() {
  GpsReadBuffer();
  float lati, lon;
  GpsReadPosition(&lati, &lon);
  Serial.println(lati);
  Serial.println(lon);
}

void GpsInit() {
  GPS_SERIAL.begin(9600, SERIAL_8N1, PIN_GPS_RX, PIN_GPS_TX);
  Serial.println("Initializing gps...ok");
}

void GpsReadBuffer() {
  uint8_t temp;

  // バッファに何かあったら読み取り解析
  while (GPS_SERIAL.available()) {
    temp = GPS_SERIAL.read();
    //    Serial.print((char)temp);
    gps.encode(temp);
//    Serial.print((char)temp);
  }
}

// 緯度経度を読み取る関数
void GpsReadPosition(float *lati, float *lon) {
  GpsReadBuffer();
  *lati = gps.location.lat();
  *lon = gps.location.lng();
}

