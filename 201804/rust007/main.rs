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

const INF : i64 = 99999999;

struct PathInfo{
    cost:i64,
    n_black:i64,
    n_white:i64,
}

fn main(){
    let inputs = read_ints();
    let (h,w,t)=(inputs[0],inputs[1],inputs[2]);
    let mut board = Vec::new();
    let mut start = (0,0);
    let mut goal = (0,0);
    let cost = 500;
    let mut costs = Vec::new();
    let mut goals = Vec::new();

    for x in 0..h{
        let mut is_black = Vec::new();

        for (y,c) in read_line().chars().enumerate(){
            is_black.push(c=='#');
            if c == 'S'{
                start = (x,y as i64);
            }else if c == 'G'{
                goal = (x,y);
            }
        }

        board.push(is_black);
    }

    for x in 0..h{
        let mut line = Vec::new();

        for y in 0..w{
            line.push(
                if (x,y)==start {
                    PathInfo{cost:0,n_black:0,n_white:0}
                }else{
                    PathInfo{cost:INF,n_black:0,n_white:0}
                }
            )
        }

        costs.push(line);
    }

    let dxs = [1,0,-1,0];
    let dys = [0,1,0,-1];

    for _ in 0..100{
        for x in 0..h{
            for y in 0..w{
                if costs[x as usize][y as usize].cost == INF{
                    continue;
                }

                for i in 0..4{
                    let dx = dxs[i];
                    let dy = dys[i];

                    if x + dx < 0 || x + dx >= h
                        || y + dy < 0 || y + dy >= w {
                        continue;
                    }
                    let nx = (x+dx) as usize;
                    let ny = (y+dy) as usize;
                    let x = x as usize;
                    let y = y as usize;

                    let inc_val = if board[nx][ny] {cost}else{1};
                    let npi = PathInfo{
                        cost:costs[x][y].cost + inc_val,
                        n_black:costs[x][y].n_black + if board[nx][ny]{1}else{0},
                        n_white:costs[x][y].n_white + if board[nx][ny]{0}else{1},
                    };


                    if costs[nx][ny].cost > npi.cost{
                        costs[nx][ny] = npi;
                    }

                    if nx == goal.0 as usize  && ny == goal.1 as usize {
                        goals.push(
                            PathInfo{
                                cost:costs[nx][ny].cost,
                                n_black:costs[nx][ny].n_black,
                                n_white:costs[nx][ny].n_white});
                    }
                }
            }
        }
    }
    

    // let mut maximum = -1;

    // for goal_cost in goals.into_iter(){
    //     let mut low = 0;
    //     let mut up = t;

    //     while low != up{
    //         let ans = (low + up)/2;
    //         let c = ans * goal_cost.n_black + goal_cost.n_white;

    //     }
    // }

    let gx = goal.0 as usize;
    let gy = goal.1 as usize;
    let temp = t - costs[gx][gy].n_white;
    let ans = temp / costs[gx][gy].n_black;

    println!("{}",costs[gx][gy].cost);
    println!("{}",costs[gx][gy].n_white);
    println!("{}",costs[gx][gy].n_black);
    println!("{}",ans);
}
