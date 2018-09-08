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

#[allow(dead_code)]
const DXS : [i64; 4] = [0,0,1,-1];
#[allow(dead_code)]
const DYS : [i64; 4]  = [1,-1,0,0];
#[allow(dead_code)]
const INF : i64 = 999999999;

////////////////////////////////////////////////

fn my_mod(n:i64,m:i64)->i64{
    ((n%m)+m)%m
}

fn gcd(n:i64,m:i64)->i64{
    let a = cmp::min(n,m);
    let b = cmp::max(n,m);
    
    if my_mod(b, a) == 0{
        a
    }else{
        gcd(my_mod(b, a),a)
    }
}

fn main(){
    let xs = read_ints();
    let (n,x) = (xs[0]as usize,xs[1]);
    let xs = read_ints().into_iter().map(|y| (y - x).abs()).collect::<Vec<_>>();
    
    if n == 1{
        println!("{}",xs[0]);
        return;
    }
    
    let mut g = gcd(xs[0],xs[1]);
        
    //println!("{:?}",xs);
    
    for x in xs{
        g = gcd(g,x);
    }
    
    println!("{}",g);
}
