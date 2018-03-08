int MARGIN = 1000;
int STEP = 75;
int N_DRAW = 5;

void setup() {
  size(512, 512, P3D);
  drawAll();
}

void drawAll() {
  background(0);

  blendMode(ADD);
  //blendMode(MULTIPLY);

  pushMatrix();

  rotateX(radians(random(30)));
  rotateY(radians(random(30)));
  rotateZ(radians(random(30)));

  for (int i = 0; i < N_DRAW; i++) {
    drawTriangles();
  }
  popMatrix();
}

void drawTriangles() {
  for (int i = -100; i < 100; i++ ) {
    for (int j = -100; j < 100; j++ ) {
      float x = (i + (j%2) * 0.5) * STEP;
      float y = j * sqrt(3) * 0.5 * STEP;

      setRandomColor();
      beginShape();
      vertex(x, y);
      vertex(x+STEP, y);
      vertex(x+0.5*STEP, y+sqrt(3) * 0.5 * STEP);
      endShape();

      setRandomColor();
      beginShape();
      vertex(x+STEP, y);
      vertex(x+1.5*STEP, y+sqrt(3) * 0.5 * STEP);
      vertex(x+0.5*STEP, y+sqrt(3) * 0.5 * STEP);
      endShape();
    }
  }
}

void setRandomColor() {
  float red, green, blue;
  int r = 20;

  red = random(52-r, 52+r)/ N_DRAW;
  green = random(106-r, 106+r)/N_DRAW;
  blue = random(134-r, 134+r)/N_DRAW;

  if (random(1) < 5e-3 ) {
    red = random(50, 255);
    green = random(50, 255);
    blue = random(50, 255);
  }
  fill(red, green, blue);
}

void draw() {
}

void mouseClicked() {
  drawAll();
}