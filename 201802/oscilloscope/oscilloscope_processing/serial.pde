Serial port = null;

int BAUDRATE = 115200;

void initSerial() {
  String[] ls = Serial.list();

  for (int i = 0; i < ls.length; i++ ) {
    println("Number " + i + ": " + ls[i]);
  }

  // Arduinoに接続
  port = new Serial(this, Serial.list()[1], BAUDRATE);
}

void serialEvent(Serial p) {
  ser_hist.append(p.read());
}

void parseHist() {
  if (ser_hist.size() < 13 )return;
  int t, ch1, ch2, cd=0;

  // 開始記号と終了記号があるかどうかの確認
  if ( !(ser_hist.get(0) == 0x00
    && ser_hist.get(1) == 0xff
    && ser_hist.get(11) == 0xff
    && ser_hist.get(12) == 0x00)) {
    ser_hist.remove(0);
    parseHist();
    return;
  }

  for (int i = 2; i < 10; i++ ) {
    cd ^= ser_hist.get(i);
  }

  if ( cd != ser_hist.get(10)) {
    ser_hist.remove(0);
    parseHist();
    return;
  }

  // 開始記号を消去
  ser_hist.remove(0);
  ser_hist.remove(0);

  t = readInt32();
  ch1 = readInt16();
  ch2 = readInt16();

  if ( t<=0 
    || ch1 < 0 || ch1 >= 1024
    || ch2 < 0 || ch2 >= 1024)
    return;


  time_hist.append(t);
  ch1_hist.append(ch1);
  ch2_hist.append(ch2);

  // 終了記号を消去
  ser_hist.remove(0);
  ser_hist.remove(0);

  parseHist();
}

int readInt32() {
  int d0 = ser_hist.get(0); 
  ser_hist.remove(0);
  int d1 = ser_hist.get(0); 
  ser_hist.remove(0);
  int d2 = ser_hist.get(0); 
  ser_hist.remove(0);
  int d3 = ser_hist.get(0); 
  ser_hist.remove(0);
  int ret = (d3 << 24) + (d2 << 16) + (d1 << 8) + d0;

  return ret;
}

int readInt16() {
  int d0 = ser_hist.get(0); 
  ser_hist.remove(0);
  int d1 = ser_hist.get(0); 
  ser_hist.remove(0);
  int ret = (d1 << 8) + d0;

  return ret;
}