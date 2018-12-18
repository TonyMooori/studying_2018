use std::io;
use std::collections::HashMap;
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
#[allow(dead_code)]
const N_PRIME : i64 = 1000000007;

////////////////////////////////////////////////

fn change(s:&mut Vec<char>,c1 : char,c2 :char){
    for i in 0..s.len(){
        if s[i] == c1 {
            s[i] = c2;
        }else if s[i] == c2{
            s[i] = c1;
        }
    }
}

fn main(){
    let s = read_line().chars().map(|c| c as usize).collect::<Vec<_>>();
    let t = read_line().chars().map(|c| c as usize).collect::<Vec<_>>();
    let mut m = vec![500;256];
    
    //let mut hm = HashMap::new();
    println!("{:?}",s);
    println!("{:?}",t);
    for i in 0..s.len(){
        if m[s[i]]  == 500{
            m[s[i]] = t[i];
        }else if m[s[i]] != t[i]{
            println!("No");
            /*
            println!("{},{},{}",
                (s[i] as u8 as char),
                (m[s[i]] as u8 as char),
                (t[i] as u8 as char));
            */
            return;
        }
    }
    
    let mut s = String::new();
    let mut t = String::new();
    
    for i in 0..256{
        if m[i] == 500{
            continue;
        }
        
        s = format!("{}{}",s,i as u8 as char);
        t = format!("{}{}",t,m[i] as u8 as char);
    }
    
    let mut s = s.chars().map(|c| c as usize).collect::<Vec<_>>();
    let mut t = t.chars().map(|c| c as usize).collect::<Vec<_>>();
    for i in 0..s.len(){
        if s[i] != target[i]{
            let c1 = s[i];
            change(&mut s,c1,target[i]);
        }
    }
    
    if s == target{
        println!("Yes");
    }else{
        println!("No");
    }
    /*
    for i in 0..256{
        if m[i] == 500{
            continue;
        }
        
        if m[i] != i{
            for j in 0..256{
                if m[j] == i{
                    m[j] = m[i];
                    m[i] = i;
                    break;
                }
            }
        }
    }
    println!("{:?}",m);
    for i in 0..256{
        
        if m[i] == 500{
            continue;
        }else if m[i] != i{
            println!("i={}={},,m[i] = {}={}",i,i as u8 as char,m[i], m[i] as u8 as char);
            println!("No");
            return;
        }
    }
    
    println!("Yes");*/
    /*
    for i in 0..s.len(){
        println!("{}",i);
        println!("{:?}",hm);
    
        match hm.get(&s[i]){
            Some(c) => if *c != target[i]{
                    println!("No");
                    println!("{},{}",*c,target[i]);
                    return;
                },
            None => {}
        }
        hm.insert(target[i].clone(),s[i].clone());
    }*/
    /*
        if s[i] != target[i]{
            let c1 = s[i];
            change(&mut s,c1,target[i]);
        }
    if s == target{
        println!("Yes");
    }else{
        println!("No");
    }*/
}
