
use std::io;
use std::collections::HashMap;
//use std::collections::BinaryHeap;
//use std::collections::HashSet;
//use std::mem; // mem::swap(&mut x,&mut y);
use std::cmp; // cmp::max,cmp::min

// Rust memo: https://hackmd.io/i0lQY1OKTDW1t66TECOmiw?both

#[allow(dead_code)]
fn read_line() -> String{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

#[allow(dead_code)]
fn read_charvec() -> Vec<char>{
    read_line().chars().collect()
}

#[allow(dead_code)]
fn read_ints() -> Vec<i64>{
    let s = read_line();
    let split:Vec<&str> = s.split(" ").collect();
    split.iter().map(|&x| x.to_string().parse().unwrap()).collect()
}

#[allow(dead_code)]
fn read_int() -> i64{
    read_ints()[0]
}

#[allow(dead_code)]
fn reverse(s: &String) -> String{
    s.chars().rev().collect::<String>()
}

#[allow(dead_code)]
fn read_lines(n:i64) -> Vec<String>{
    let mut xs = Vec::new();

    for _i in 0..n{
        xs.push(read_line());
    }

    xs
}

#[allow(dead_code)]
fn read_ints2(n:i64) -> Vec<i64>{
    read_lines(n).iter().map(|x| x.parse().unwrap()).collect()
}

#[allow(dead_code)]
fn arange(start:i64,end:i64,step:i64) -> Vec<i64>{
    let mut ret = Vec::new();
    let mut i = start;

    while i < end{
        ret.push(i);
        i += step;
    }

    ret
}

#[allow(dead_code)]
const DXS : [i64; 4] = [0,0,1,-1];
#[allow(dead_code)]
const DYS : [i64; 4]  = [1,-1,0,0];
#[allow(dead_code)]
const INF : i64 = 999999999;

////////////////////////////////////////////////

fn get_first(s:&String) -> char{
    s.chars().nth(0).unwrap()
}
fn get_last(s:&String) -> char{
    let n = s.len()-1;
    s.chars().nth(n).unwrap()
}

fn main(){
    let n = read_int();
    let words = (0..n).map(|_|read_line()).collect::<Vec<String>>();
    let mut hm = HashMap::new();
    let mut c = get_last(&words[0]);
    
    for i in 0..n as usize{
        if hm.get(&words[i]).is_some(){
            println!("No");
            return;
        }else{
            hm.insert(words[i].clone(),true);
        }
    }
    
    for i in 1..n as usize{
        if get_first(&words[i]) != c{
            println!("No");
            //println!("{},{}",c,get_first(&words[i]));
            
            return;
        }
        
        c = get_last(&words[i]);
        
    }
    println!("Yes");
}

