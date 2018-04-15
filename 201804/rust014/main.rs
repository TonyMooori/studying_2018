use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::io;

const MEM_SIZE : usize = 1024;

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
    
    exec(contents);
}

fn getc() -> char{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().chars().nth(0).unwrap()
}

fn exec(code:String){
    let mut memory = vec![0 as u8;MEM_SIZE];
    let code : Vec<char> = code.chars().collect();
    let mut i = 0;
    let mut ptr = 0;
    let mut bracket = Vec::new();

    while i < code.len(){
        match code[i]{
            '>'=>   ptr=(ptr+1)%MEM_SIZE,
            '<'=>   ptr=(ptr-1+MEM_SIZE)%MEM_SIZE,
            '+'=>   memory[ptr] = ((memory[ptr] as u32 + 256 + 1)&0xff) as u8,
            '-'=>   memory[ptr] = ((memory[ptr] as u32 + 256 - 1)&0xff) as u8,
            '.'=>   print!("{}",memory[ptr] as char),
            ','=>   memory[ptr] = getc() as u8,
            '['=>   {
                if memory[ptr] == 0 {
                    // skip
                    let mut bracket_num = 0;
                    while i < code.len(){
                        if code[i] == '['{
                            bracket_num += 1;
                        }else if code[i] == ']'{
                            bracket_num -= 1;
                        }
                        if bracket_num == 0{
                            break;
                        }
                        i += 1;
                    }
                }else{
                    bracket.push(i);
                }
            },
            ']'=>   {
                match bracket.pop(){
                    Some(v) => i = v,
                    None => panic!("Cannot find \'[\' before \']\'"),
                }
                continue;
            },
            _ =>    {}
        }
        i += 1;
    }
}