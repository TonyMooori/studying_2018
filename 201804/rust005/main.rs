use std::io;
//use std::collections::HashMap;
//use std::collections::BinaryHeap;
//use std::collections::HashSet;
//use std::mem; // mem::swap(&mut x,&mut y);
// use std::cmp; // cmp::max,cmp::min
 
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
 
// https://beta.atcoder.jp/contests/abc084/tasks/abc084_d
// prime number

fn prime_board(n:i64)->Vec<bool>{
    let mut ret = vec![true;(n+1) as usize];
 
    ret[0]=false;
    ret[1]=false;
    for i in 2..(n+1){
        if ret[i as usize] {
            let mut j = 2*i;
 
            while j < n+1{
                ret[j as usize]=false;
                j +=i;
            }
        }
    }
 
    ret
}
 
fn main(){
    let n : usize = 100009;
    let board = prime_board(n as i64);
    let mut total = 0;
    let mut arr = vec![0 as i32;n/2];
 
    // for i in 0..100 {
    //     if board[i]{
    //         println!("{},",i);
    //     }
    // }
 
    for i in 0..n/2{
        let m1 = i * 2 + 1;
        let m2 = (m1+1)/2;
        let flag = board[m1] && board[m2];
 
        arr[i] = total;
 
        if flag {
            total += 1;
        }
    }
 
    let q = read_int();
 
    for _ in 0..q{
        let input = read_ints();
        let (l,r) = (input[0],input[1]);
 
        let l_idx = (l-1)/2;
        let r_idx = (r-1)/2;
 
        let ans0 = arr[r_idx as usize+1]-arr[l_idx as usize];
        println!("{}",ans0);
    }
}