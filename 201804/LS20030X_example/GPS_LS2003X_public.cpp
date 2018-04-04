#include "GPS_LS2003X.h"

// constructor
GPS_LS2003X::GPS_LS2003X(const int PIN_TX, const int PIN_RX) {

  // initialize all variables
  _ser = SoftwareSerial(PIN_TX, PIN_RX);
  strcpy(_line , "");
  _length = 0;
  _is_east = _is_north = true;
  _latitude = _longitude = _altitude = 0.0;
  _hour = _minute = _second = 0;

  // set modes
  begin();
}

// begin connection
void GPS_LS2003X::begin() {
  begin(BAUDRATE_9600, INTERVAL_1000MS);
}

// begin connection
void GPS_LS2003X::begin(const GPS_Baudrate_mode_t baud_mode,
                        const GPS_Interval_mode_t interval_mode) {

  set_baudrate(baud_mode);
  set_interval(interval_mode);
  send_data("PMTK314,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0");
  _ser.flush();
}

// update method ( call this method constantly )
void GPS_LS2003X::update() {
  while (_ser.available()) {
    // get data
    const char c = _ser.read();

    if ( c == '\n' ) {
      split_data(_line);
      _length = 0;
      continue;
    }
    if( c == '$' ){
      _length = 0;
    }

    if ( _length == LINE_LENGTH - 1 )
      _length = 0;

    if ( _length == 0 && c != '$' )
      continue;

    _line[ _length ] = c;
    _length++;
    _line[ _length ] = '\0';
  }
}

// set interval to get data
void GPS_LS2003X::set_interval(const GPS_Interval_mode_t mode) {
  const int32_t interval = intervals[ (int)mode ];
  char buff[32];

  sprintf(buff, "PMTK220,%ld", interval);
  send_data(buff);
}

// set baud rate of connection
void GPS_LS2003X::set_baudrate(const GPS_Baudrate_mode_t mode) {
  const int32_t baudrate = baudrates[ (int)mode ];
  char buff[32];

  _ser.begin(baudrate);
  sprintf(buff, "PMTK215,%ld", baudrate);
  send_data(buff);
}

// is on east side
bool GPS_LS2003X::get_is_east() const {
  return _is_east;
}

// is on north side
bool GPS_LS2003X::get_is_north() const {
  return _is_north;
}

// get latitude
float GPS_LS2003X::get_latitude() const {
  return _latitude;
}

// get longitude
float GPS_LS2003X::get_longitude() const {
  return _longitude;
}

// get altitude
float GPS_LS2003X::get_altitude() const {
  return _altitude;
}

// get hour(UTF+0)
int8_t GPS_LS2003X::get_hour() const {
  return _hour;
}

// get minutes
int8_t GPS_LS2003X::get_minute() const {
  return _minute;
}

// get second
int8_t GPS_LS2003X::get_second() const {
  return _second;
}
