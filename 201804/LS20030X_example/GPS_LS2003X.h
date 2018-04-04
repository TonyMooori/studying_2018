#ifndef GPS_LS2003X_HEADER
#define GPS_LS2003X_HEADER

#include <SoftwareSerial.h>
#include <Arduino.h>
#include <string.h>
#include <stdio.h>

#define LINE_LENGTH     128
#define N_CHUNK_SIZE    16

typedef enum {
  INTERVAL_100MS  =   0,  // get gps data  10Hz
  INTERVAL_200MS  =   1,  // get gps data   5Hz
  INTERVAL_500MS  =   2,  // get gps data   2Hz
  INTERVAL_1000MS =   3,  // get gps data   1Hz
  INTERVAL_2000MS =   4   // get gps data 0.5Hz
} GPS_Interval_mode_t;

typedef enum {
  BAUDRATE_4800       = 0,  // 4800bps
  BAUDRATE_9600       = 1,  // 9600bps
  BAUDRATE_19200      = 2,  // 19200bps
  BAUDRATE_38400      = 3,  // 38400bps
  BAUDRATE_57600      = 4,  // 57600bps
  BAUDRATE_115200     = 5   // 115200bps
} GPS_Baudrate_mode_t;

// baud rate array
static const int32_t baudrates[] = { 4800, 9600, 19200, 38400, 57600, 115200};
// interval array
static const int32_t intervals[] = { 100, 200, 500, 1000, 2000};

class GPS_LS2003X {
  public:
    // constructor
    GPS_LS2003X(const int PIN_TX, const int PIN_RX);

    // begin connection
    void begin();
    void begin( const GPS_Baudrate_mode_t baud_mode, const GPS_Interval_mode_t interval_mode);
    // call this method constantly
    void update();

    // set interval to get data
    void set_interval(const GPS_Interval_mode_t mode);
    // set baud rate of connection
    void set_baudrate(const GPS_Baudrate_mode_t mode);

    bool get_is_east() const;     // is on east side
    bool get_is_north() const;    // is on north side
    float get_latitude() const;   // get latitude
    float get_longitude() const;  // get longitude
    float get_altitude() const;   // get altitude
    int8_t get_hour() const;      // get hour(UTF+0)
    int8_t get_minute() const;    // get minutes
    int8_t get_second() const;    // get second

  private:

    // send command to gps sensor
    void send_data(const char *code);
    // calclate checksum
    uint8_t get_checksum(const char *str) const;
    // error detection by checksum
    bool check_error(const char *str) const;
    // get datas from GPGGA code
    void split_data(const char *str);
    // hhmmss -> h,m,s
    void extract_time(int32_t val, uint8_t *hour, uint8_t *mini, uint8_t *sec) const;
    // dddmm.mmmm -> ddd.dddd
    float extract_LL(const float val) const;
    // comma separated string to string array
    int split(const char *str, char output[N_CHUNK_SIZE][N_CHUNK_SIZE])const;

    SoftwareSerial _ser = SoftwareSerial(2, 3);
    char _line[LINE_LENGTH];
    uint8_t _length;
    bool _is_east;
    bool _is_north;
    float _latitude; // 緯度
    float _longitude; // 経度
    float _altitude;
    uint8_t _hour, _minute, _second;
};

#endif
