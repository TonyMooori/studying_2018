import processing.serial.*;
import controlP5.*;

int MAX_SIZE = 2048;


void setup() {
  size(640, 480);

  initData();
  initGui();
  initSerial();
}


void draw() {
  background(0); 
  fill(0);
  parseHist();
  deleteOldData();
  drawMesh();
  drawGraph();
}