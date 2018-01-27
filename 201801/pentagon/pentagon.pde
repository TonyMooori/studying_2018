void setup() {
  size(640, 480, P3D);
  noStroke();
}

void draw() {
  myclear();

  if (random(1) < 5e-1)
    drawPentagon(random(width), random(height), 50, random(2*PI), color(255));
  filter(BLUR, 2);
}

void myclear() {
  fill(32, 32, 32, 10);
  rect(0, 0, width, height);
  //blendMode(ADD);

  //beginShape();
  //vertex(0,0);
  //vertex(width,0);
  //vertex(width,height);
  //vertex(0,height);
  //endShape(CLOSE);
}

void drawPentagon(float x, float y, float r, float phi, int c) {
  float s, t;

  //blendMode(ADD);
  pushMatrix();
  translate(x, y);

  beginShape();
  for (int i = 0; i < 5; i++ ) {
    fill(c);
    s = r * cos(i*2*PI/5.0+phi);
    t = r * sin(i*2*PI/5.0+phi);
    vertex(s, t);
  }
  endShape(CLOSE);

  popMatrix();
}