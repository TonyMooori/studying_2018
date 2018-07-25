// アルゴリズムパズル 6. 指数え
use std::io;

fn read_line() -> String{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn read_int() -> i64{
    read_ints()[0]
}

const FINGER_ORDER : [&str;8] =[
    "人差し指", "親指",    "人差し指",
    "中指",     "薬指",    "小指",
    "薬指",     "中指"];

fn main(){
    let n = read_int();
    let i = (n as usize) % FINGER_ORDER.len();

    println!("{}",FINGER_ORDER[i]);
}