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

fn count(b:i64,n:i64,k:i64) -> i64{
    // bが引数のbであとき，a%b>=kなるaの個数を返す関数

    if b <= k {
        return 0;
    }

    let chunk_size = cmp::max(0,b-k);// 1塊の中のtrueの数
    let n_chunk = n / b;    // ちゃんとあれしてるchunkの数
    let n_rest = n % b;     // 残った部分
    let candy = cmp::max(0,n_rest - k + 1);

    candy + chunk_size * n_chunk
}

fn main(){
    let inputs = read_ints();
    let (n,k) = (inputs[0],inputs[1]);

    if k==0{
        println!("{}",n*n);
        return;
    }

    let ans = (1..n+1).into_iter()
            .map(|b| count(b,n,k))
            .fold(0,|s,i|s+i);
    
    println!("{}",ans);
}