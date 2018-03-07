use std::io;
//use std::collections::HashMap;
//use std::collections::BinaryHeap;
//use std::collections::HashSet;
//use std::mem; // mem::swap(&mut x,&mut y);
use std::cmp; // cmp::max,cmp::min

// Rustメモ: https://hackmd.io/i0lQY1OKTDW1t66TECOmiw?both

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

////////////////////////////////////////////////

fn calc(v:i64,p:i64)->i64{
    v*(100+p)/100
}

fn solve() -> bool{
    // http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1192&lang=jp
    let inputs = read_ints();
    let (x,y,s) = (inputs[0],inputs[1],inputs[2]);

    if x == 0{
        return false;
    }

    let vs : Vec<i64> = (0..(s+1)*(s+1))
        .into_iter()
        .filter(|a|{
            let left = a / (s+1) + 1;
            let right = a % (s+1) + 1;
            let left_p = calc(left,x);
            let right_p = calc(right,x);
            let ret = (left_p+right_p) == s;

            ret
        })
        .map(|a|{
            let left = a / (s+1) + 1;
            let right = a % (s+1) + 1;
            let left_p = calc(left,y);
            let right_p = calc(right,y);

            left_p+right_p
        })
        .collect();
    let max_value = vs
        .into_iter()
        .fold(-1,|a,b| cmp::max(a,b));
    println!("{}",max_value);

    true
}

fn main(){
    while solve(){}
}
