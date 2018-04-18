#ifndef MY_SERIAL_H
#define MY_SERIAL_H

#include<Arduino.h>

class MySerial {
  public:
    MySerial(int pin, uint32_t baudrate) {
      pin_ = pin;
      delay_us_ = 1000000 / baudrate - 12;
      pinMode(pin_, OUTPUT);
      digitalWrite(pin_, HIGH);
    }

    void write(byte b) {
      byte checksum = 0;
      digitalWrite(pin_, LOW);
      delayMicroseconds(delay_us_);
      
      for (int i = 0 ; i < 8 ; i++) {
        digitalWrite(pin_, b & 0x01);
        delayMicroseconds(delay_us_);
        checksum = checksum ^ (b & 0x01);
        b = b >> 1;
      }
      digitalWrite(pin_, checksum);
      delayMicroseconds(delay_us_);
      digitalWrite(pin_, HIGH);
      delayMicroseconds(delay_us_);
    }

  private:
    int pin_;
    uint32_t delay_us_;
};

// atmega328p-pu 20MHz 50ns/clk
// https://www.avrfreaks.net/forum/attiny85-softwareserial-atmel-studio-7
// tunedDelayとかnewsoftwareserialの遺産とか

#endif

