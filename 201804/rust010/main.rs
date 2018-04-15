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

fn display(board:&Vec<Vec<bool>>){
    for i in 0..100{
        for j in 0..100{
            print!("{}",if board[i][j] {'#'} else {'.'});
        }
        println!("");
    }
}

const DXS : [i64; 4] = [0,0,1,-1];
const DYS : [i64; 4]  = [1,-1,0,0];

fn fill(board:&Vec<Vec<bool>>,mut label:&mut Vec<Vec<i64>>,x:usize,y:usize,c:i64){
    label[x][y] = c;
    let f = board[x][y];

    for i in 0..4{
        let dx = DXS[i];
        let dy = DYS[i];
        let nx = (x as i64) + dx;
        let ny = (y as i64) + dy;

        if nx < 0 || 100 <= nx || ny < 0 || 100 <= ny {
            continue;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if f == board[nx][ny]{
            fill(&board,&mut label,nx,ny,c);
        }
    }
}

fn count(board:&Vec<Vec<bool>>)->(i64,i64){
    let mut label = vec![vec![-1;100];100];
    let mut c = 0;
    let mut b = 0;
    let mut w = 0;

    for i in 0..100{
        for j in 0..100{
            if label[i][j] < 0 {
                fill(&board,&mut label,i,j,c);
                c += 1;

                if board[i][j]{
                    b+=1;
                }else{
                    w+=1;
                }
            }
        }
    }

    (w,b)
}

fn solve(board:&Vec<Vec<bool>>,w:i64,b:i64){
    let (ww,bb) = count(&board);
    let change_type = if 
    if ww == w && bb == b{
        display(board);
        return;
    }
}

fn main(){
    let input = read_ints();
    let (w,b) = (input[0],input[1]);
    let mut board = vec![vec![false;100];100];

    for i in 0..100{
        for j in 0..100{
            board[i][j] = (i+j) % 2 == 0;
        }
    }
    
    solve(&mut board,w,b);
}
