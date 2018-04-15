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

////////////////////////////////////////////////

const INF : i64 = 9999999999;

fn calc(is_black:&Vec< Vec<bool> >,s:(usize,usize),e:(usize,usize),xx:i64) -> i64{
    let (w,h) = (is_black[0].len(),is_black.len());
    let mut costs = vec![vec![INF;w];h];
    let dxs = [0,1,0,-1];
    let dys = [-1,0,1,0];

    costs[s.0][s.1]=0;

    for _ in 0..100{
        for x in 0..h{
            for y in 0..w{
                if costs[x][y] == INF{
                    continue;
                }
                for i in 0..4{
                    let dx = dxs[i];
                    let dy = dys[i];

                    let nx = dx + x as i64;
                    let ny = dy + y as i64;

                    if nx < 0 || h as i64 <= nx || ny < 0 || w  as i64 <= ny{
                        continue;
                    }

                    let nx = nx as usize;
                    let ny = ny as usize;

                    costs[nx][ny] = cmp::min(
                        costs[nx][ny],
                        costs[x][y] + if is_black[x][y] {xx}else{1}
                    );
                }
            }
        }
    }

    costs[e.0][e.1]
}

fn main(){
    let input = read_ints();
    let (h,_w,t) = (input[0],input[1],input[2]);
    let mut is_black  = Vec::new();
    let mut s = (0,0);
    let mut e = (0,0);

    for x in 0..h as usize{
        let line = read_line();
        let mut temp = Vec::new();
        
        for (y,c) in line.chars().enumerate(){
            temp.push(c=='#');

            if c == 'S'{
                s = (x,y);
            }else if c == 'G'{
                e = (x,y);
            }
        }

        is_black.push(temp);
    }

    let mut left = 1;
    let mut right = t+1;

    while left != right - 1{
        let x = (left+right)/2;
        let total_time = calc(&is_black,s,e,x);
        if total_time > t {
            right = x;
        }else{
            left = x;
        }
        // println!("{}",total_time);
        // println!("{},{}",left,right)
    }

    println!("{}",left);
}

