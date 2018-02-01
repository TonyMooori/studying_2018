use std::io;
//use std::collections::HashMap;
//use std::collections::BinaryHeap;
//use std::collections::HashSet;
//use std::mem;
use std::cmp;


fn read_line() -> String{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn read_charvec() -> Vec<char>{
    read_line().chars().collect()
}

fn read_ints() -> Vec<i64>{
    let s = read_line();
    let split:Vec<&str> = s.split(" ").collect();
    split.iter().map(|&x| x.to_string().parse().unwrap()).collect()
}

fn read_int() -> i64{
    read_ints()[0]
}

//fn reverse(s: &String) -> String{
//    s.chars().rev().collect::<String>()
//}

//////////////////////////////////////////////////////////////////////

// https://arc002.contest.atcoder.jp/tasks/arc002_3
fn solve(xs:&Vec<i32>,left:&i32,right:&i32) -> i32{
    let mut ys = vec![10000; xs.len()];
    let l1 = left % 4;
    let l2 = left / 4;
    let r1 = right % 4;
    let r2 = right / 4;
    
    ys[0] = 1;
    ys[1] = if (r1 == xs[0] && r2 == xs[1]) || (l1 == xs[0] && l2 == xs[1]) { 1 } else { 2 };
    
    for i in 2..xs.len(){
        if (xs[i-1] == r1 && r2 == xs[i]) || (xs[i-1] == l1 && l2 == xs[i] ){
            ys[i] = cmp::min(cmp::min(ys[i-2]+1,ys[i-1]+1),ys[i]);
        }else{
            ys[i] = cmp::min(ys[i],ys[i-1]+1);
        }
    }
    
    ys[xs.len()-1]
}

fn main(){
    let n = read_int() as i32;
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
    let mut m = 10000;
    
    if n <= 2 {
        m = 1;
    }else{
        for i in 0..256 as i32{
            let left = i % 16;
            let right = i / 16;
            
            if left<=right{
                m = cmp::min(m,solve(&xs,&left,&right));
            }
        }
    }
    
    
    println!("{}",m);
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
