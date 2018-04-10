use std::io;
//use std::collections::HashMap;
//use std::collections::BinaryHeap;
//use std::collections::HashSet;
//use std::mem; // mem::swap(&mut x,&mut y);
// use std::cmp; // cmp::max,cmp::min


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

fn main(){
    let inputs = read_ints();
    let (n,k) = (inputs[0],inputs[1]);
    let ts = read_ints(); // number of theorem
    let bs = read_ints(); // behavior
    // the total number of theorem t=0 to to=k-1(waked up at t=0)
    let mut did = (0..k)
        .into_iter()
        .fold(0,|s,i| s + ts[i as usize]);
    // the total number of theorem t=0 to to=k-1(dont waked up)
    let mut didnt = (0..k)
        .into_iter()
        .fold(0,|s,i| s + bs[i as usize] * ts[i as usize] );
    // the total number of theorem t=0 to to=n-1(dont waked up)
    let didnt_total = (0..n)
        .into_iter()
        .fold(0,|s,i| s + bs[i as usize] * ts[i as usize] );
    let mut max_val = did-didnt;

    // println!("{:?}",ts);
    // println!("{:?}",bs);
    // println!("nke");

    for i in 0..(n-k) as usize{
        let ku = k as usize;

        did = did - ts[i] + ts[i+ku];
        didnt = didnt - bs[i]*ts[i] + bs[i+ku]*ts[i+ku];

        if max_val < did-didnt{
            max_val = did-didnt;
            // println!("updated at i={}",i);
            // println!("max_val = {}",max_val);
            // println!("didnt_total = {}",didnt_total);
        }
    }

    println!("{}",didnt_total + max_val);
}