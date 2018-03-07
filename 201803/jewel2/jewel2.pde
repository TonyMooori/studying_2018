void setup() {
  size(512, 512);

  background(5,50,5);
  noStroke();

  blendMode(ADD);
  int N = 16;
  for (int i = 0; i < N; i++ ) {
    for (int j = 0; j < N; j++ ) {
      //float x = random(1)*width;
      //float y = random(1)*height;
      float x = i*(width / N);
      float y = j*(height / N);
      drawJewel(x, y);
    }
  }
}

void drawJewel(float x, float y) {
  float r = random(1)*50 + 50;
  int n = (int)(random(3, 6));
  drawPolygons(x, y, n, r);
}

void drawPolygons(float x, float y, int count, float r) {
  pushMatrix();
  beginShape();
  translate(x, y);
  float red = random(5, 50);
  float green = random(50, 255);
  float blue = random(5, 50);
  int angle = 180 / (count - 2);
  fill(red, green, blue);
  for (int i = 0; i < count; i++ ) {
    rotate(radians(random(angle/2)+angle/2));
    vertex(random(-r, r), random(-r, r));
  }
  endShape();
  popMatrix();
}

void draw() {
  if (mousePressed) {
    if (mouseButton == LEFT ) {
      blendMode(ADD);
    } else {
      blendMode(MULTIPLY);
    }
    drawJewel(mouseX, mouseY);
  }
}