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
    let xs = read_ints();
    let (h,w) = (xs[0] as usize,xs[1] as usize);
    
    let mut mat = (0..h).map(|_| read_ints()).collect::<Vec<Vec<i64>>>();
    //let mut checked = vec![vec![false;w];h];
    
    let mut ans = Vec::new();
    
    let mut cnt = 0;
    for i in 0..h{
        for j in 0..w{
            if mat[i][j] % 2 == 1{
                if j == w-1{
                    if i == h-1{
                        // もうむり
                    }else{
                        // した
                        mat[i][j]-=1;
                        mat[i+1][j]+=1;
                        ans.push((i+1,j+1,i+2,j+1));
                        cnt+=1;
                    }
                }else{
                    // となり
                    mat[i][j]-=1;
                    mat[i][j+1]+=1;
                    ans.push((i+1,j+1,i+1,j+2));
                    cnt+=1;
                }
            }
            
        }
    }
    
    println!("{}",cnt);
    for x in ans{
        println!("{} {} {} {}",x.0,x.1,x.2,x.3);
    }
}   