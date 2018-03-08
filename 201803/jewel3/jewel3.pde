int N_POINT = 500;
int MARGIN = 500;
int MAX_DIST = 500;
int MIN_DIST = 100;
FloatList xs;
FloatList ys;

void setup() {
  size(512, 512);
  drawAll();
}

void drawAll() {
  xs = new FloatList();
  ys = new FloatList();

  for (int i = 0; i < N_POINT; i++ ) {
    xs.append(map(random(1), 0, 1, -MARGIN, width+MARGIN));
    ys.append(map(random(1), 0, 1, -MARGIN, height+MARGIN));
  }

  background(0);
  blendMode(ADD);
  //blendMode(MULTIPLY);
  for (int i = 0; i < 100; i++ ) {
    beginShape();
    float red = random(10,15);
    float green = random(10,25);
    float blue = random(10,15);
    fill(red, green, blue);

    float x = xs.get((int)random(xs.size()));
    float y = xs.get((int)random(ys.size()));

    vertex(x, y);
    setNearVertex(x, y);
    setNearVertex(x, y);

    endShape();
  }
}

void setNearVertex(float x, float y) {
  float nx = xs.get((int)random(xs.size()));
  float ny = xs.get((int)random(ys.size()));

  for (int i = 0; i < 100; i++) {
    if ( MIN_DIST < dist(x, y, nx, ny)  && dist(x, y, nx, ny) < MAX_DIST) {
      break;
    }
    nx = xs.get((int)random(xs.size()));
    ny = xs.get((int)random(ys.size()));
  }

  vertex(nx, ny);
}

void draw() {
}

void mouseClicked() {
  drawAll();
}