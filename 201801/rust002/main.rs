use std::io;
//use std::collections::HashMap;
//use std::collections::BinaryHeap;
use std::collections::HashSet;
//use std::mem;
//use std::cmp;


fn read_line() -> String{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn read_ints() -> Vec<i64>{
    let s = read_line();
    let split:Vec<&str> = s.split(" ").collect();
    split.iter().map(|&x| x.to_string().parse().unwrap()).collect()
}

fn read_int() -> i64{
    read_ints()[0]
}

//////////////////////////////////////////////////////////////////////

fn reverse(s: &String) -> String{
    s.chars().rev().collect::<String>()
}

fn solve(){
    let mut myset = HashSet::new();
    let s = read_line();

    for i in 1..s.len(){
        let first = s[0..i].to_string();
        let rfirst = reverse(&first);
        let rest  = s[i..s.len()].to_string();
        let rrest = reverse(&rest);
        
        myset.insert(format!("{}{}",rest,first));
        myset.insert(format!("{}{}",rest,rfirst));
        myset.insert(format!("{}{}",rrest,first));
        myset.insert(format!("{}{}",rrest,rfirst));
        
        myset.insert(format!("{}{}",first,rest));
        myset.insert(format!("{}{}",first,rrest));
        myset.insert(format!("{}{}",rfirst,rest));
        myset.insert(format!("{}{}",rfirst,rrest));
    }
    
    println!("{}",myset.len());

}

fn main(){
    let n = read_int();
    for _i in 0..n{
        solve();
    }
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


use std::mem;

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



*/
