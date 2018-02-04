#define PIN_CHANNEL_1 A0
#define PIN_CHANNEL_2 A1

#define BAUDRATE  115200

int16_t ch1, ch2;
uint32_t time_ms;
uint8_t check_digit;

void setup() {
  Serial.begin(BAUDRATE);
  pinMode(PIN_CHANNEL_1, INPUT);
  pinMode(PIN_CHANNEL_2, INPUT);
}

void loop() {
  check_digit = 0;
  ch1 = analogRead(PIN_CHANNEL_1);
  ch2 = analogRead(PIN_CHANNEL_2);
  time_ms = millis();

  Serial.write(0x00);
  Serial.write(0xff);
  check_digit = sendData((uint8_t*)&time_ms, 4, check_digit);
  check_digit = sendData((uint8_t*)&ch1, 2, check_digit);
  check_digit = sendData((uint8_t*)&ch2, 2, check_digit);
  check_digit = sendData((uint8_t*)&check_digit, 1, check_digit);
  Serial.write(0xff);
  Serial.write(0x00);
}

uint8_t sendData(uint8_t *data, uint16_t n, uint8_t cd) {
  for (int i = 0 ; i < n ; i++ ) {
    Serial.write(data[i]);
    cd ^= data[i];
  }
  return cd;
}

