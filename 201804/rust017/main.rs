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

fn main(){
    let inputs = read_ints();
    let  a = inputs[0];
    let  b = inputs[1];
    let  ab = inputs[2];
    let mut x = inputs[3];
    let mut y = inputs[4];
    let mut ans = 0;
    
    if a+b>2*ab{
        let m = cmp::min(x,y);
        ans += m * ab * 2;
        x -= m;
        y -= m;
        // println!("buy ab {}",m);
    }

    if a > 2*ab{
        ans += x * ab * 2;
        // println!("bu ab {}",x);
        y = cmp::max(0,y-x);
        x = 0;
    }
    if b > 2*ab{
        ans += y * ab * 2;
        // println!("bu ab {}",y);
        x = cmp::max(0,x-y);
        y = 0;
    }

    ans += x * a;
    ans += y * b;

    println!("{}",ans);

}