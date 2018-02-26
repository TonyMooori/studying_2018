use std::io;
//use std::collections::HashMap;
//use std::collections::BinaryHeap;
//use std::collections::HashSet;
//use std::mem; // mem::swap(&mut x,&mut y);
//use std::cmp; // cmp::max,cmp::min

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

//////////////////////////////////////////////////////////////////////

// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2018&lang=jp

fn solve() -> bool{
    let xs = read_ints();
    let (n,m,p) = (xs[0],xs[1],xs[2]);

    if n==0{
        return true;
    }

    let xs = read_ints2(n);
    let n_win = xs[(m-1) as usize];
    let s = xs.iter().fold(0,|x,y| x + y);

    if n_win == 0{
        println!("0");
    }else{
        let rest = s * (100-p) / n_win;

        println!("{}",rest);
    }
    return false;
}

fn main(){
    loop{
        if solve (){
            break;
        }
    }
}





/*
DOCS

///////////////// Vec /////////////////

let v = vec![0;2]; // [0,0]
let sum_x = xs.iter().fold(0,|s,i|s+i); // xs : Vec<i64>

s.chars().nth(i as usize).unwrap();

let xs : Vec<i32> = read_charvec().iter().map(|&c| 
    if c == 'A'{
        0
    }else if c == 'B'{
        1
    }else if c == 'X'{
        2
    }else{
        3
    }
).collect();

// search
let i = match xs.iter().find(|&&m| m==0){
    Some(v) => true,
    None => false,
};

///////////////// HashMap /////////////////

let mut h = HashMap::new();
let a = 5;
h.insert(a,true);
match h.get(&a){
    Some(v) => println!("{}",v),
    None    => println!("none"),
}

///////////////// swap /////////////////

let mut x = 5;
let mut y = 1;
mem::swap(&mut x,&mut y);

///////////////// priority queue /////////////////

// https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
let mut heap = BinaryHeap::new();

heap.push(1);
heap.push(5);
heap.push(2);

assert_eq!(heap.pop(), Some(5));
assert_eq!(heap.pop(), Some(2));
assert_eq!(heap.pop(), Some(1));
assert_eq!(heap.pop(), None);

let mut vs = heap.into_vec();
vs.reverse();

*/
