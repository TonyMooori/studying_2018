
IntList time_hist;
IntList ch1_hist;
IntList ch2_hist;
IntList ser_hist;

PrintWriter output = null;
import java.io.File;

void initData() {
  time_hist = new IntList();
  ch1_hist = new IntList();
  ch2_hist = new IntList();
  ser_hist = new IntList();
}

void deleteOldData() {
  while (time_hist.size() != 0 
    && time_hist.get(0)>time_hist.get(time_hist.size()-1)) {
    ch1_hist.remove(0);
    ch2_hist.remove(0);
    time_hist.remove(0);
  }

  while (ch1_hist.size() > MAX_SIZE) {
    saveData(time_hist.get(0), ch1_hist.get(0), ch2_hist.get(1));
    ch1_hist.remove(0);
    ch2_hist.remove(0);
    time_hist.remove(0);
  }
}

void saveData(int t, int ch1, int ch2) {
  if (saving)
    saveString(t + "," + ch1+ "," + ch2);
}

void saveString(String s) {
  if (output == null ) {
    makeLogFile();
  }
  output.println(s);
}

void makeLogFile() {
  int i = 0;
  while (true) {
    File f = new File(sketchPath() + ".\\data" + i + ".csv");
    
    if (f.exists()==false)break;
    i+=1;
  }
  output = createWriter(sketchPath() + ".\\data" + i + ".csv");
}