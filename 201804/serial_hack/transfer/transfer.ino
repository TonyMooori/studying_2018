void setup() {
  // put your setup code here, to run once:
  Serial.begin(1000, SERIAL_8E1);
}

void loop() {
  // put your main code here, to run repeatedly:
//  Serial.println("Hello, world!");
  Serial.write(85);
  Serial.write(64);
  delay(1000);
}
