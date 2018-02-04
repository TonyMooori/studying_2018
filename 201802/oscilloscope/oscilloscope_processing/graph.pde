int WIDTH = 640;
int HEIGHT = 480;


int GRAPH_MARGIN_SIDE = 50;
int GRAPH_MARGIN_TOP = 50;
int GRAPH_MARGIN_BOTTOM = 150;

int GRAPH_LEFT = GRAPH_MARGIN_SIDE;
int GRAPH_RIGHT = WIDTH-GRAPH_MARGIN_SIDE;
int GRAPH_TOP = GRAPH_MARGIN_TOP;
int GRAPH_BOTTOM = HEIGHT - GRAPH_MARGIN_BOTTOM;

float HREF_VOLTAGE = 5.0;

void drawMesh() {
  textAlign(CENTER);
  int n = ch1_hist.size();

  // 大枠の描画
  stroke(128, 128, 128);
  rect(GRAPH_LEFT-1, GRAPH_TOP-1, GRAPH_RIGHT-GRAPH_LEFT+2, GRAPH_BOTTOM-GRAPH_TOP+2);

  // 横線の描画
  for (int i = 0; i <= 5; i++) {
    float y = map(i, 0, 5, GRAPH_BOTTOM+1, GRAPH_TOP-1);

    stroke(128, 128, 128);
    line(GRAPH_LEFT, y, GRAPH_RIGHT, y);

    stroke(255);
    fill(255);
    text(i+"V", 20, y);
  }

  for (int i = 0; i < 10; i++) {
    float x = map(i, 0, 10, GRAPH_LEFT-1, GRAPH_RIGHT+1);

    stroke(128, 128, 128);
    line(x, GRAPH_BOTTOM, x, GRAPH_TOP);
  }

  if (n <= 1)return;

  stroke(255);
  fill(255);
  float t = (time_hist.get(n-1)-time_hist.get(0))*1e-3;
  text(nf(0, 2, 3) + "s", GRAPH_LEFT, GRAPH_BOTTOM+20);
  text(nf(t, 2, 3) + "s", GRAPH_RIGHT, GRAPH_BOTTOM+20);
}

void drawGraph() {
  int n = ch1_hist.size();
  if (n <= 1)return;

  int t_min = time_hist.get(0);
  int t_max = time_hist.get(n-1);

  for (int i = 0; i < n-1; i++) {
    float temp1 = +ch1_offset.getValue()*1024.0 / HREF_VOLTAGE;
    float temp2 = +ch2_offset.getValue()*1024.0 / HREF_VOLTAGE;
    float ch1_0 = ch1_hist.get(i) + temp1;
    float ch1_1 = ch1_hist.get(i+1) + temp1;
    float ch2_0 = ch2_hist.get(i) + temp2;
    float ch2_1 = ch2_hist.get(i+1) + temp2;
    int t0 = time_hist.get(i);
    int t1 = time_hist.get(i+1);

    float x0 = map(t0, t_min, t_max, GRAPH_LEFT, GRAPH_RIGHT);
    float x1 = map(t1, t_min, t_max, GRAPH_LEFT, GRAPH_RIGHT);
    float ch1_y0 = map(ch1_0, 0, 1024, GRAPH_BOTTOM, GRAPH_TOP);
    float ch1_y1 = map(ch1_1, 0, 1024, GRAPH_BOTTOM, GRAPH_TOP);
    float ch2_y0 = map(ch2_0, 0, 1024, GRAPH_BOTTOM, GRAPH_TOP);
    float ch2_y1 = map(ch2_1, 0, 1024, GRAPH_BOTTOM, GRAPH_TOP);

    stroke(0, 255, 0);
    line(x0, constrain(ch1_y0, GRAPH_TOP, GRAPH_BOTTOM), 
      x1, constrain(ch1_y1, GRAPH_TOP, GRAPH_BOTTOM));
    stroke(255, 0, 0);
    line(x0, constrain(ch2_y0, GRAPH_TOP, GRAPH_BOTTOM), 
      x1, constrain(ch2_y1, GRAPH_TOP, GRAPH_BOTTOM));
  }
}