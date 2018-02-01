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

//////////////////////////////////////////////////////////////////////

// https://arc001.contest.atcoder.jp/tasks/arc001_2
fn solve(n:i64) -> i64{
    let mut count = 0;
    let mut xs = vec![n;1];
    
    loop{
        let i = match xs.iter().find(|&&m| m==0){
            Some(v) => *v,
            None => -1
        };
        
        if i!=-1{
            break;
        }
        
        let n = xs.len();
        
        for j in 0..n{
            let x = xs[j];
            xs.push(x+10);
            xs.push(x-10);
            xs.push(x+5);
            xs.push(x-1);
            xs.push(x-5);
            xs.push(x+1);
        }
        
        count += 1;
    }
    
    count
}

fn main(){
    let xs = read_ints();
    let (a,b) = (xs[0],xs[1]);
    let n = (a-b).abs();
    println!("{}",solve(n));
}



/*
DOCS

let mut h = HashMap::new();
let a = 5;
h.insert(a,true);
match h.get(&a){
    Some(v) => println!("{}",v),
    None    => println!("none"),
}


let mut x = 5;
let mut y = 1;
mem::swap(&mut x,&mut y);


let v = vec![0;2]; // [0,0]

s.chars().nth(i as usize).unwrap();


// priority queue
// https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
use std::collections::BinaryHeap;
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

// 文字列をVec<char>として読み取ってVec<i32>に変換
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

// 検索
let i = match xs.iter().find(|&&m| m==0){
    Some(v) => true,
    None => false,
};
*/
