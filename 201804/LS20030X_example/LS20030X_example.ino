
#include <Arduino.h>
#include <SoftwareSerial.h>
#include "GPS_LS2003X.h"

//SoftwareSerial ser(2,3);
GPS_LS2003X gps(2, 3);

void setup() {
  Serial.begin(9600);
  delay(2000);
  gps.begin();
  gps.set_interval(INTERVAL_500MS);
  gps.set_baudrate(BAUDRATE_9600);
  Serial.print("setup function is finished.\n");
}

void loop() {

  Serial.print(gps.get_is_east());
  Serial.print("\t");
  Serial.print(gps.get_is_north());
  Serial.print("\t");
  Serial.print(gps.get_latitude());
  Serial.print("\t");
  Serial.print(gps.get_longitude());
  Serial.print("\t");
  Serial.print(gps.get_altitude());
  Serial.print("\t");
  Serial.print(gps.get_hour());
  Serial.print("\t");
  Serial.print(gps.get_minute());
  Serial.print("\t");
  Serial.print(gps.get_second());
  Serial.print("\n");


  gps.update();
  delay(100);
}

