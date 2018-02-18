// 秋月電子販売サイト
// １ＭｂｉｔＩスケアＣシリアルＥＥＰＲＯＭ　２４ＦＣ１０２５－Ｉ／Ｐ
// http://akizukidenshi.com/catalog/g/gI-03570/
// データシート
// en: http://akizukidenshi.com/download/24fc1025-ip.pdf
// jp: http://akizukidenshi.com/download/24fc1025-ip.pdf
// 使ってみた
// http://www.geocities.jp/bokunimowakaru/diy/arduino/eeprom.html

#include "EEPROM_24FC1025.h"

#define PIN_EEPROM_A0 2
#define PIN_EEPROM_A1 3

// EEPROMを扱うクラス
EEPROM24FC1025 eeprom(PIN_EEPROM_A0, PIN_EEPROM_A1);

void setup() {
  Serial.begin(9600);
  eeprom.Reset();
}

void loop() {
  // sample data
  const uint8_t write_data[8] = {3, 1, 4, 1, 5, 9, 2, 6};
  // buffer for reading
  uint8_t read_buffer[8];
  // start address of data (24FC1025 0x0000 - 0x3FFFF)
  eeprom.I2CScan();
  eeprom.MemoryTest();
}
