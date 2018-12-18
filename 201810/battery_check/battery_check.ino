void setup() {
  // put your setup code here, to run once:
  Serial.begin(19200);
  pinMode(A0, INPUT);
  pinMode(A1, INPUT);
}

void loop() {
  // put your main code here, to run repeatedly:
  uint32_t val0 = 0;
  uint32_t val1 = 0;
  for (int i = 0 ; i < 1000 ; i++ ) {
    val0 += analogRead(A0);
    val1 += analogRead(A1);
    delay(1);
  }
  float v0 = val0 / 1000.0 / 1024;
  float v1 = val1 / 1000.0 / 1024;

  v1 *= 7.77 / 0.75;
  v0 *= 7.70 / 0.76;

  Serial.print(millis());
  Serial.print(",");
  Serial.print(v0);
  Serial.print(",");
  Serial.print(v1);
  Serial.println();
}
