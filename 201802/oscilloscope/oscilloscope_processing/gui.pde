
ControlP5 cp5;
Slider slider;
Button save_btn;
Slider ch1_offset;
Slider ch2_offset;

boolean gui_enable = false;
boolean saving = false;

void initGui() {
  cp5 = new ControlP5(this);

  save_btn = cp5.addButton("save")
    .setLabel("START SAVEING")
    .setPosition(50, 10)
    .setSize(100, 30);

  slider = cp5.addSlider("max_size")
    .setLabel("MAX DATA NUM")
    .setRange(10, 8192)
    .setValue(25)
    .setPosition(160, 10)
    .setSize(100, 30);

  ch1_offset = cp5.addSlider("ch1_offset")
    .setLabel("ch1 offset")
    .setRange(-5, +5)
    .setValue(0)
    .setPosition(GRAPH_LEFT, GRAPH_BOTTOM+50)
    .setSize(100, 30);

  ch2_offset = cp5.addSlider("ch2_offset")
    .setLabel("ch2 offset")
    .setRange(-5, +5)
    .setValue(0)
    .setPosition(GRAPH_LEFT, GRAPH_BOTTOM+85)
    .setSize(100, 30);

  gui_enable = true;
}

void controlEvent(ControlEvent e) {
  if (gui_enable==false)return;
  String name = e.getController().getName();

  if ( name == "save") {
    saving = !saving;
    save_btn.setLabel(saving?"STOP SAVING":"START SAVING");
    if (saving==false) {
      output.flush();
      output.close();
      output = null;
    }
  } else if (name == "max_size") {
    MAX_SIZE = (int)(slider.getValue());
    slider.setValue(MAX_SIZE);
  }
}