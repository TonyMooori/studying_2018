const int N = 16;
volatile int vals[N];
volatile uint32_t times[N];
volatile int count = 0;

void setup() {
  // put your setup code here, to run once:
  Serial.begin(115200);
  pinMode(2, INPUT);
}

void loop() {
  // put your main code here, to run repeatedly:
  char buff[256];
  
  Serial.println("start");
  count = 0;
  attachInterrupt(0, logging, CHANGE);
  while (count < N);
  detachInterrupt(0);

  uint32_t t_old = times[0];
  for (int i = 0 ; i < N ; i++) {
    sprintf(buff, "%08ld,%08ld,%d", times[i] - times[0],times[i]-t_old, vals[i]);
    t_old = times[i];
    Serial.println(buff);
  }

//  delay(10000);
}

void logging() {
  if (count == N)return;

  times[count] = micros();
  vals[count] = digitalRead(2);
  count++;
}

