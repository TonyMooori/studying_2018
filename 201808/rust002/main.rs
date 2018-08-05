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

#[derive(Clone,Debug)]
struct Quiz{
    n_point : i64,
    n_quiz : i64,
    n_bonus : i64,
}
/*
fn solve(xs:Vec<Quiz>,g:i64){
    println!("{}",m);
}*/

fn solve(xs:Vec<Quiz>,g:i64)->i64{
    let mut m = 999999999999;
    
    for i in  0..(1<<xs.len()){
        let mut rest = g;
        let mut count = 0;
        
        // 全解き
        for j in 0..xs.len(){
            if (i >> j)&1 == 0{
                continue;
            }
            rest -= xs[j].n_point * xs[j].n_quiz + xs[j].n_bonus;
            count += xs[j].n_quiz;
        }
        
        // 少しだけ解く
        for j in 0..xs.len(){
            if rest <= 0{
                break;
            }
            // すでに解いてるやつはとかない(解けない)
            if (i >> j)&1 != 0{
                continue;
            }else if rest <= xs[j].n_point * xs[j].n_quiz{
                // 幾つかとけばよい
                let u = (rest + xs[j].n_point - 1)/xs[j].n_point;
                count += u;
                rest -= u * xs[j].n_point;
                break;
            }else{
                // 全部解かねばならない
                rest -= xs[j].n_point * xs[j].n_quiz + xs[j].n_bonus;
                count += xs[j].n_quiz;
            }
        }
        
        if rest<=0{
            m = cmp::min(m,count);
        }
    }
    
    m
}

/*
fn solve(mut xs:Vec<Quiz>,g:i64)->i64{
    let ys = xs.iter()
        .map(|x| (x.n_point * x.n_quiz + x.n_bonus) as f64 / (x.n_quiz as f64))
        .collect::<Vec<f64>>();
    let zs = xs.iter()
        .map(|x| (x.n_point * x.n_quiz + x.n_bonus))
        .collect::<Vec<i64>>();
    let max_idx = {
        let mut idx = 0;
        for i in 0..ys.len(){
            if ys[idx] <= ys[i]{
                idx = i;
            }
        }
        idx 
    };
    
    if g > zs[max_idx]{
        let n_solved = xs[max_idx].n_quiz;
        println!("we solved {:?}",xs[max_idx]);
        xs.remove(max_idx);
        
        n_solved + solve(xs,g-zs[max_idx])
    }else{
        let mut m = 999999999;
        
        for i in  0..(1<<xs.len()){
            let mut sum = g;
            let mut n_solved = 0;
            
            for j in 0..xs.len(){
                if (i >> j)&1 == 0{
                    continue;
                }
                
                if sum < xs[j].n_point * xs[j].n_quiz{
                    // 何個か解けばよいだけ
                    let n_s = sum / xs[j].n_point;
                    
                    n_solved += n_s;
                    sum -= n_s * xs[j].n_point;
                    
                    if sum <= 0{
                        n_solved += 1;
                        sum -= xs[j].n_point;
                    }
                }else{
                    // 全部とけば良い
                    sum -= xs[j].n_point * xs[j].n_quiz + xs[j].n_bonus;
                    n_solved += xs[j].n_quiz;
                }
                
                if sum <= 0{
                    m = cmp::min(n_solved,m);
                    break;
                }
            }
        }
        
        m
    }
}*/

fn main(){
    let inputs = read_ints();
    let (d,g) = (inputs[0] as usize,inputs[1]);
    
    let mut xs = (0..d)
        .map(|i| (i,read_ints()) )
        .map(|p| Quiz{
            n_point:p.0 as i64 * 100+100,
            n_quiz:p.1[0],
            n_bonus:p.1[1]
        })
        .collect::<Vec<Quiz>>();
    
    xs.reverse();
    //println!("{:?}",xs);
    
    println!("{}",solve(xs,g));
}