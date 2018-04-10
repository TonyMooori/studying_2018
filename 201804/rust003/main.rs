use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

fn main(){
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("The only one command line argument is needed.");
    }

    let filename = &args[1];
    let file = match File::open(filename){
        Ok(v) => v,
        Err(_) => panic!("Cannot open file: {}",filename)
    };
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    match reader.read_to_string(&mut contents){
        Ok(_) => {},
        Err(_) => panic!("Cannot read file: {}",filename)
    };
    println!("{}",contents);

}