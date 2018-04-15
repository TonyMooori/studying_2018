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

fn pattern(v: &Vec<i64>) -> Vec<i64>{
    let mut ret = vec![0;128];
    
    for x in v.iter(){
        ret[*x as usize] += 1;
    }

    ret
}

fn search_same(v:&Vec<i64>,vs:&Vec< Vec<i64> >) -> Vec<usize>{
    let pat = pattern(v);
    let mut ret = Vec::new();

    for i in 0..vs.len(){
        let pat1 = pattern(&vs[i]);
        if pat == pat1 {
            ret.push(i);
        }
    }

    ret
}

fn to_symmetric(board:&mut Vec<Vec<i64>>){
    
}

fn main(){
    let inputs = read_ints();
    let (h,w) = (inputs[0],inputs[1]);
    let mut board = Vec::new();

    for _ in 0..h as usize{
        let v : Vec<i64> = read_line().chars().map(|c| c as i64).collect();
        board.push(v);
    }

    for i in 0..h as usize{
        let patterns = search_same(&board[i],&board);
        println!("{:?}",patterns);

        if patterns.len() == 1{
            continue;
        }
    }

    println!("NO");
}