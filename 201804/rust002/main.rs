use std::io;
//use std::collections::HashMap;
//use std::collections::BinaryHeap;
//use std::collections::HashSet;
//use std::mem; // mem::swap(&mut x,&mut y);
use std::cmp; // cmp::max,cmp::min


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

fn read_board(n:i64) -> Vec<i64>{
    (0..n)
    .into_iter()
    .map(|_| read_line())
    .fold(String::new(),|s,i| format!("{}{}",s,i))
    .chars()
    .into_iter()
    .map(|c| if c == '0' {0} else {1})
    .collect()
}

fn calc_diff(piece:&Vec<i64>,target:&Vec<i64>) ->i64{
    let mut sum = 0;

    for i in 0..piece.len(){
        sum = sum + if piece[i] != target[i]{ 1 } else { 0 };
    }

    sum
}

fn main(){
    let n = read_int();
    let mut piece = Vec::new();
    let odd_str : Vec<i64> = (0..n*n)
        .into_iter()
        .map(|i| i%2)
        .collect();
    let even_str : Vec<i64> = (0..n*n)
        .into_iter()
        .map(|i| 1-i%2)
        .collect();
    let mut ret = n*n*4;

    piece.push(read_board(n));
    read_line();
    piece.push(read_board(n));
    read_line();
    piece.push(read_board(n));
    read_line();
    piece.push(read_board(n));

    for i in 0..16{
        let flags = (0..4)
            .into_iter()
            .map(|x| if i & (1 << x) == 0 {0} else {1})
            .fold(0,|s,i|s+i);

        if flags != 2{
            continue;
        }
        let diff = (0..4)
            .into_iter()
            .map(|x| calc_diff(&piece[x],if i&(1<<x) == 0 {&even_str} else{&odd_str}))
            .fold(0,|s,i|s+i);
        // println!("{}",diff);
        ret = cmp::min(ret,diff);
    }
    // println!("{}",piece[0]);
    // println!("{}",even_str);
    // println!("{}",odd_str);
    println!("{}",ret);
}