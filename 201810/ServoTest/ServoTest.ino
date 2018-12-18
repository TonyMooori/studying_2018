#include <Servo.h>

Servo s;
String str;
void setup() {
  // put your setup code here, to run once:
  Serial.begin(19200);
  s.attach(4);
  str = "";
}

void loop() {
  // put your main code here, to run repeatedly:
  int angle = (millis() / 1000) % 2 == 0 ? 50 : 130;
  s.write(angle);
  Serial.println(angle);
}
