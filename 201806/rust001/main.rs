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


fn make_cost(change_cost: &Vec<Vec<i64>>,color_map: &Vec<Vec<i64>>,n : usize) -> Vec<i64>{
    let c = change_cost.len();
    let mut ret = vec![0 as i64;c];
    //eprintln!("make-cost start");
    //eprintln!("n={},c={}",n,c);

    for i in 0..color_map.len(){
        for j in 0..color_map[0].len(){
            if (i+j)%3 == n{
                let color_from = (color_map[i][j]-1) as usize;
                
                for color_to in 0..c{
                    ret[color_to] += change_cost[color_from][color_to];
                }
            }
        }
    }
    //eprintln!("make-cost end");

    ret
}

fn solve(costs:Vec<Vec<i64>>)->i64{
    let c = costs[0].len();
    let mut min_val = 999999999999;

    for c1 in 0..c{
        for c2 in 0..c{
            if c1 == c2{
                continue;
            }

            for c3 in 0..c{
                if c1 == c3 || c2 == c3{
                    continue;
                }

                min_val = cmp::min(
                    min_val,
                    costs[0][c1] + costs[1][c2] + costs[2][c3]
                );

            }
        }
    }

    min_val

}

fn main(){
    let inputs = read_ints();
    let (n,c) = (inputs[0],inputs[1]);

    let change_cost : Vec<Vec<i64>> = (0..c)
        .map(|_| read_ints())
        .collect();
    let color_map : Vec<Vec<i64>> = (0..n)
        .map(|_| read_ints())
        .collect();
    
    let types_change_cost : Vec<Vec<i64>> =
        (0..3)
        .map(|n| make_cost(&change_cost,&color_map,n))
        .collect();

    println!("{}",solve(types_change_cost));
}
