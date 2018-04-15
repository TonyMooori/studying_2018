use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::io;

enum Command{
    IncData,
    DecData,
    IncPointer,
    DecPointer,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

struct Data{
    cmd: Command,
    num: i64,
    paren_idx: usize,
}

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

fn count(code:&Vec<char>,i:&mut usize,c:char)->i64{
    if c == '[' || c == ']'{
        *i = *i + 1;
        1
    }else if *i == code.len(){
        0
    }else if c == code[*i]{
        *i = *i + 1;
        1 + count(code,i,c)
    }else{
        0
    }
}

fn parse(code:String)->Vec<Data>{
    let code : Vec<char> = code.chars().collect();
    let mut ret : Vec<Data> = Vec::new();
    let mut i = 0;
    let mut bracket = Vec::new();

    while i < code.len(){
        let mut temp = Data{ 
            cmd : Command::IncData,
            num : 0,
            paren_idx : 0
        };
        let c = code[i];

        temp.num = count(&code,&mut i,c);

        match c {
            '>'=> temp.cmd = Command::IncPointer,
            '<'=> temp.cmd = Command::DecPointer,
            '+'=> temp.cmd = Command::IncData,
            '-'=> temp.cmd = Command::DecData,
            '.'=> temp.cmd = Command::Output,
            ','=> temp.cmd = Command::Input,
            '['=>   {
                temp.cmd = Command::LoopStart;
                bracket.push(ret.len());
            },
            ']'=> match bracket.pop(){
                    Some(v) => {
                        temp.cmd = Command::LoopEnd;
                        temp.paren_idx = v;
                        ret[v].paren_idx = ret.len();
                    },
                    None => panic!("Cannot find \'[\' before \']\'"),
                },
            _ => continue,
        }

        ret.push(temp);
    }
    ret
}

fn exec(code:String){
    let code = parse(code);
    let mut i = 0;
    let mut memory = vec![0 as u8;MEM_SIZE];
    let mut ptr = 0;
    
    while i < code.len(){
        match code[i].cmd{
            Command::IncPointer  => ptr = ((ptr as i64+code[i].num)&(MEM_SIZE as i64 -1)) as usize,
            Command::DecPointer  => ptr = ((ptr as i64-code[i].num)&(MEM_SIZE as i64 -1)) as usize,
            Command::IncData     => memory[ptr] = ((memory[ptr] as i64+code[i].num)&0xff) as u8,
            Command::DecData     => memory[ptr] = ((memory[ptr] as i64-code[i].num)&0xff) as u8,
            Command::Output      => for _ in 0..code[i].num { print!("{}",memory[ptr] as char) },
            Command::Input       => for _ in 0..code[i].num { memory[ptr] = getc() as u8},
            Command::LoopStart   => if memory[ptr]==0 {i=code[i].paren_idx;},
            Command::LoopEnd     => i = code[i].paren_idx-1,
        }
        i = i + 1;
    }
}
