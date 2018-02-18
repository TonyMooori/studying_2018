#ifndef EEPROM_24FC1025
#define EEPROM_24FC1025

#include <Arduino.h>
#include <Wire.h>

#define N_CHUNK_FOR_TEST    64

class EEPROM24FC1025 {
  public:

    EEPROM24FC1025(int pin_A0, int pin_A1) {
      pin_A0_ = pin_A0;
      pin_A1_ = pin_A1;
      Reset();
    }

    // reset instance
    void Reset() {
      pinMode(pin_A0_, OUTPUT);
      pinMode(pin_A1_, OUTPUT);
    }

    // write datas to eeprom
    // datas      : the pointer of data which you want to write.
    // n_data     : the number of data
    // addr       : the address where you want to write. (0x0000-0xFFFF)
    // chip_select: the eeprom chip you want to write.(0or1or2or3)
    void WriteBytes(const uint8_t *datas, const uint16_t n_data, const  uint16_t addr, const uint8_t chip_select) {
      // write by 1 bytes
      for (uint32_t i = 0 ; i < n_data ; i ++ ) {
        WriteByte(datas[i], addr + i, chip_select);
      }
    }

    // write datas to eeprom
    // data       : the byte data which you want to write.
    // addr       : the address where you want to write. (0x0000-0xFFFF)
    // chip_select: the eeprom chip you want to write.(0or1or2or3)
    void WriteByte(const uint8_t data, const  uint16_t addr, const uint8_t chip_select) {
      const uint8_t i2c_addr = i2c_addr_[chip_select];

      // enable chip you want to write
      digitalWrite(pin_A0_, chip_select % 2);
      digitalWrite(pin_A1_, chip_select / 2);

      delayMicroseconds(10);

      // write data
      Wire.beginTransmission(i2c_addr);
      Wire.write(highByte(addr));
      Wire.write(lowByte(addr));
      Wire.write(data);

      Wire.endTransmission();

      delay(5);
    }

    // read datas from eeprom
    // datas      : a place to store the read data
    // n_data     : the number of data
    // addr       : the address where you want to read. (0x0000-0xFFFF)
    // chip_select: the eeprom chip you want to read.(0or1or2or3)
    void ReadBytes(uint8_t *datas, const uint16_t n_data, const uint16_t addr, const uint8_t chip_select) {
      // under 16 datas -> read at once
      if (n_data <= 16) {
        uint8_t i2c_addr = i2c_addr_[chip_select];

        // enable chip you want to read
        digitalWrite(pin_A0_, chip_select % 2);
        digitalWrite(pin_A1_, chip_select / 2);

        delayMicroseconds(10);

        Wire.beginTransmission(i2c_addr);
        Wire.write(highByte(addr));
        Wire.write(lowByte(addr));
        Wire.endTransmission();

        Wire.requestFrom((int)i2c_addr, n_data);

        for (uint32_t i = 0 ; i < n_data ; i++) {
          datas[i] = (uint8_t)Wire.read();
        }
      } else {
        // over 16 datas -> read at several times
        for (uint32_t i = 0 ; i < n_data ; i += 16 ) {
          ReadBytes(datas + i, min(n_data - i, 16), addr + i, chip_select);
        }
      }
    }

    // read datas from eeprom
    // data       : a place to store the read data
    // addr       : the address where you want to read. (0x0000-0xFFFF)
    // chip_select: the eeprom chip you want to read.(0or1or2or3)
    void ReadByte(uint8_t *data, const uint16_t addr, const uint8_t chip_select) {
      ReadBytes(data, 1, addr, chip_select);
    }

    // list i2c devices
    void I2CScan() {
      int ret;
      Serial.println("i2c scan started");
      delay(100);
      for (uint8_t i = 1 ; i < 128 ; i++ ) {
        Wire.beginTransmission(i);
        ret = Wire.endTransmission();
        if (ret == 0) {
          Serial.print("I2C device found at ");
          Serial.print("0x");
          if (i < 16) {
            Serial.print("0");
          }
          Serial.println(i, HEX);
        }
      }
      Serial.println("i2c scan ended");
      Serial.println();
    }

    // test reading/writing eeprom
    void MemoryTest() {
      uint8_t data[N_CHUNK_FOR_TEST];
      const uint32_t seed = micros();
      randomSeed(seed);
      const uint16_t addr = random(0xFFFF - N_CHUNK_FOR_TEST);
      const uint8_t chip_select = random(4);

      // display the seed of random number, address. selected chip
      Serial.print("random seed = ");
      Serial.println(seed);
      Serial.print("addr = ");
      Serial.println(addr);
      Serial.print("chip_select = ");
      Serial.println(chip_select);

      // make datas to write
      for (uint32_t i = 0 ; i < N_CHUNK_FOR_TEST ; i++ ) {
        data[i] = random(256);
      }

      Serial.println("writing datas...(if stoped, you should check pulling up sda/scl.");
      WriteBytes(data, N_CHUNK_FOR_TEST, addr, chip_select);
      delay(1000);
      Serial.println("reading datas...");
      delay(1000);
      ReadBytes(data, N_CHUNK_FOR_TEST, addr, chip_select);

      // display datas which you read
      //      for (int i = 0 ; i < N_CHUNK ; i++ ) {
      //        if (data[i] < 16) {
      //          Serial.print("0x0");
      //        } else {
      //          Serial.print("0x");
      //        }
      //        Serial.print(data[i], HEX);
      //        Serial.print(" ");
      //      }
      //      Serial.println();

      randomSeed(seed);
      random(0xFFFF - N_CHUNK_FOR_TEST);
      random(4);

      for (int i = 0 ; i < N_CHUNK_FOR_TEST ; i++ ) {
        // data which wrote
        const uint8_t expected = random(256);

        // compare data which wrote and data which read
        if ( expected != data[i]) {
          Serial.print("Error at ");
          Serial.print(i);
          Serial.print(",expected ");
          Serial.print(expected);
          Serial.print(", we got ");
          Serial.println(data[i]);
        }
      }
      Serial.println("memtest fin");
      Serial.println();
    }

  private:

    // the arduino's pin connected to A0 pin of EEPROM
    int pin_A0_;
    // the arduino's pin connected to A1 pin of EEPROM
    int pin_A1_;
    // i2c address of eeprom(order is [A0,A1] =[0,0],[0,1],[1,0],[1,1])
    const uint8_t i2c_addr_[4] = {0x50, 0x51, 0x52, 0x53};

};

#endif

