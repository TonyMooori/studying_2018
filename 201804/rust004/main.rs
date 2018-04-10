use std::io;
//use std::collections::HashMap;
//use std::collections::BinaryHeap;
//use std::collections::HashSet;
//use std::mem; // mem::swap(&mut x,&mut y);
use std::cmp; // cmp::max,cmp::min

// Rustメモ: https://hackmd.io/i0lQY1OKTDW1t66TECOmiw?both

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

fn calc_min_cost(cost:&mut Vec<Vec<i64>>){
    for _ in 0..10{
        for i0 in 0..10{
            for i1 in 0..10{
                for i2 in 0..10{
                    cost[i0][i2] = cmp::min(
                        cost[i0][i2],
                        cost[i0][i1]+cost[i1][i2]);
                }
            }
        }
    }
}

fn main(){
    let mut cost = Vec::new();
    let mut board = Vec::new();
    let input = read_ints();
    let (h,w) = (input[0],input[1]);
    let mut ans = 0;

    for _ in 0..10{
        cost.push(read_ints());
    }
    for _ in 0..h{
        board.push(read_ints());
    }
    calc_min_cost(&mut cost);

    for i in 0..h as usize{
        for j in 0..w as usize{
            if board[i][j] != -1 {
                ans += cost[board[i][j] as usize][1];
            }
        }
    }
    println!("{}",ans);
}
