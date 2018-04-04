#include "GPS_LS2003X.h"

// send command to gps sensor
void GPS_LS2003X::send_data(const char *code) {
  const uint8_t checksum = get_checksum(code);
  char buff[8];

  sprintf(buff, "%02X", checksum);

  _ser.print("$");
  _ser.print(code);
  _ser.print("*");
  _ser.println(buff);
}

// comma separated string to string array
int GPS_LS2003X::split(const char *str, char output[N_CHUNK_SIZE][N_CHUNK_SIZE]) const {
  int comma_index[N_CHUNK_SIZE + 1];
  int n_comma = 1;
  const int len = strlen(str);

  comma_index[0] = 0;

  // search comma index
  for (int i = 0 ; i < len && n_comma < N_CHUNK_SIZE ; i++ ) {
    if ( str[i] == ',' ) {
      comma_index[n_comma] = i;
      n_comma++;
    }
  }
  comma_index[n_comma] = len;
  n_comma++;

  // get split data
  for (int i = 0 ; i < n_comma - 1; i++ ) {
    int sub_len = comma_index[i + 1] - comma_index[i] - 1;
    memcpy(output[i], &str[comma_index[i] + 1], sub_len);
    output[i][sub_len] = '\0';
  }

  return n_comma - 1;
}

// get datas from GPGGA code
void GPS_LS2003X::split_data(const char *str) {
  char buff[N_CHUNK_SIZE][N_CHUNK_SIZE];
  int n_comma;

  // check checksum
  if ( check_error(str) == false )
    return ;

  // count the number of comma
  n_comma = split(str, buff);
  if ( n_comma != 15 )
    return;

  if ( strcmp(buff[0], "GPGGA") != 0 )
    return;

  extract_time((int32_t)atof(buff[1]), &_hour, &_minute, &_second);
  _latitude = extract_LL(atof(buff[2]));
  _is_north = strcmp(buff[3], "N") == 0;
  _longitude = extract_LL(atof(buff[4]));
  _is_north = strcmp(buff[5], "E") == 0;
  _altitude = atof(buff[9]);
}

// dddmm.mmmm -> ddd.dddd
float GPS_LS2003X::extract_LL(const float val) const {
  float div_100 = floor(val / 100.0f);
  return div_100 + (val - 100.0f * div_100)  / 60.0;
}

// hhmmss -> h,m,s
void GPS_LS2003X::extract_time(int32_t val, uint8_t *hour, uint8_t *mini, uint8_t *sec) const {
  *sec = val % 100; val = val / 100;
  *mini = val % 100; val = val / 100;
  *hour = val % 100;
}

// error detection by checksum
bool GPS_LS2003X::check_error(const char *str) const {
  char buff[128] = {0};
  char checksum_str[3];   // the end of str
  char checksum_buff[3];   // the result of calculation of checksum
  uint8_t checksum;
  int star_at;            // the index of '*';
  const int len = strlen(str);

  star_at = strchr(str, '*') - str + 1;
  if ( star_at < 0 )
    return false;
  
  memcpy( buff, &str[1], star_at - 2);
  buff[star_at] = '\0';
    
  memcpy( checksum_str, &str[star_at], 2 );
  checksum_str[2] = '\0';

  checksum = get_checksum(buff);
  sprintf(checksum_buff, "%02X", checksum);

  return strcmp(checksum_buff, checksum_str) == 0;
}

// calclate checksum
uint8_t GPS_LS2003X::get_checksum(const char *str) const {
  uint8_t ret = 0;
  const uint8_t len = strlen(str);
  for (int i = 0 ; i < len ; i++ )
    ret ^= str[i];
  return ret;
}
