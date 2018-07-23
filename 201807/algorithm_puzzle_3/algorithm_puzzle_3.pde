

void setup() {
  size(640, 480);
}

void draw() {
  background(0);
  make_triangle(millis()/100);
}

void make_triangle(int n) {
  int n1 = n / 2;
  int n2 = n - n1;

  split_triangle(n1, 1, 1, 1, height-1, width-1, height-1);
  split_triangle(n2, 1, 1, width-1, 1, width-1, height-1);
}

void split_triangle(
  int n, float x0, float y0, float x1, float y1, float x2, float y2) {
  // 頂点(x1,y1)で直角となている直角三角形を描画
  triangle(x0, y0, x1, y1, x2, y2);
  
  if (n<=1)return;

  // 次の直角三角形の座標を計算
  float d1 = dist(x0, y0, x1, y1);
  float d3 = dist(x2, y2, x0, y0);
  float k = (d1/d3)*(d1/d3);
  float x3 = k * (x2-x0) + x0;
  float y3 = k * (y2-y0) + y0;

  int n1 = n / 2;
  int n2 = n - n1;

  split_triangle(n1, x0, y0, x3, y3, x1, y1);
  split_triangle(n2, x2, y2, x3, y3, x1, y1);
}