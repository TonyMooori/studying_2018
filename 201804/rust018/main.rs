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

fn main(){
    let inputs = read_ints();
    let (n,c) = (inputs[0],inputs[1]);
    let (dist,val) = {
        let mut d = Vec::new();
        let mut v = Vec::new();

        for _ in 0..n {
            let xs = read_ints();
            d.push(xs[0]);
            v.push(xs[1]);
        }

        (d,v)
    };

    
    if n > 100{
        println!("-1");
        return;
    }

    let (inc_turn,ans1) = {
        let mut s = 0;
        let mut v = Vec::new();
        let mut temp = 0;

        for i in 0..n as usize{
            s += val[i];
            v.push(s-dist[i]);
            temp = cmp::max(v[i],temp);
        }

        (v,temp)
    };
    let (dec_turn,ans2) = {
        let mut s = 0;
        let mut v = Vec::new();
        let mut temp = 0;
        for i in (0..n as usize).rev() {
            s += val[i];
            v.push(s-(c-dist[i]));
            temp = cmp::max(v[n as usize -1-i],temp);
        }

        (v,temp)
    };

    let ans3 = {
        let mut temp = 0;

        for i in 0..n as usize{
            for j in 0..n as usize{
                let inc = i;
                let dec = n as usize- j - 1;

                if inc >= dec{
                    continue;
                }

                temp = cmp::max(
                    temp,
                    inc_turn[i] + dec_turn[j] - dist[inc]
                );

                temp = cmp::max(
                    temp,
                    inc_turn[i] + dec_turn[j] - (c-dist[dec])
                );
            }
        }

        temp
    };

    println!("{}",cmp::max(ans1,cmp::max(ans3,ans2)));
}
