use std::time::{Duration,Instant};

macro_rules! measure{
    ($x:expr) => {
        {
            let start = Instant::now();
            let result = $x;
            let end = start.elapsed();
            println!("elapsed time {}.{:03}",end.as_secs(),end.subsec_nanos() / 1000_000);
            result
        }
    };
}

fn main(){
    measure!(test1());
    measure!(test2());
}

fn test1(){
    let mut s = String::new();

    for i in 0..100_000{
        s = format!("{}{}",s,"tttt".to_string());
    }
}


fn test2(){
    let mut s = String::new();

    for i in 0..100_000{
        s= s + "tttt";
    }
}
