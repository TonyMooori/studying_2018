#include "MySerial.h"

const int PIN_TX = 2;
MySerial ser(PIN_TX,1000);

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
}

void loop() {
  // put your main code here, to run repeatedly:
  ser.write(85);
  ser.write(64);
  delay(1000);
}
