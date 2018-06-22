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

fn calc(xs:&Vec<Vec<i64>>,tp: i64, m : usize)->i64{
    let sign = vec![
        if (tp   ) & 1 == 1 {1}else{-1},
        if (tp>>1) & 1 == 1 {1}else{-1},
        if (tp>>2) & 1 == 1 {1}else{-1},
    ];

    let mut ys : Vec<i64> = xs.iter()
        .map(|x| x[0] * sign[0] + x[1] * sign[1] + x[2] * sign[2])
        .collect();
    ys.sort();
    ys.reverse();

    let mut ret = 0;
    for i in 0..m{
        ret += ys[i];
    }

    ret
}

fn main(){
    let input = read_ints();
    let (n,m) = (input[0],input[1]);

    let mut xs : Vec<Vec<i64>> = (0..n).into_iter()
        .map(|_| read_ints())
        .collect();

    let mut ans = 0;
    for i in 0..8{
        ans = cmp::max(ans,calc(&xs,i,m as usize));
    }
    println!("{}",ans);
}
