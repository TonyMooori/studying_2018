use std::io;
//use std::collections::HashMap;
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

////////////////////////////////////////////////

fn check_all(a:&i64,b:&i64,x:&i64) -> bool{
    let c = a * b;

    // println!("----------{}",x);
    for i in 0..*x{
        let aa = if i+1 < *a { i+1 } else { i + 2 };
        let bb = if x-*b > i {*x-i} else {*x-i-1};

        // println!("check ({},{}):{}",aa,bb,aa*bb);
        if aa * bb >= c {
            return false;
        }
    }

    true
}


fn check_middle(a:&i64,b:&i64,x:&i64) -> bool{
    
    let c = a * b;
    let x0 = ((a*b) as f64).sqrt() as i64 - 10;
    // println!("----------{}",x);
    for i in x0..x0+20{
        let aa = if i+1 < *a { i+1 } else { i + 2 };
        let bb = if x-*b > i {*x-i} else {*x-i-1};

        // println!("check ({},{}):{}",aa,bb,aa*bb);
        if aa * bb >= c {
            return false;
        }
    }

    true
}

fn solve(a:i64,b:i64) -> i64{
    if a == 1 && b == 1 {
        return 0;
    }

    let mut xmax = cmp::max(2,(((a*b) as f64).sqrt() * 2.0 + 20.0) as i64);

    if xmax < 50 {
        while ! check_all(&a,&b,&xmax){
            xmax = xmax - 1;
        }
    }else {
        while ! check_middle(&a,&b,&xmax){
            xmax = xmax - 1;
        }
    }
    xmax-1
}

fn main(){
    let q = read_int();
    for _ in 0..q{
        let input =read_ints();
        let (a,b) = (input[0],input[1]);
        println!("{}",solve(a,b));
    }
}
