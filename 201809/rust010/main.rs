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
#[allow(dead_code)]
const N_PRIME : i64 = 1000000007;

////////////////////////////////////////////////

fn calc_factors(mut m:i64) -> Vec<(i64,i64)>{
    let n = 31700;
    let mut primes = vec![true;n];
    primes[0] = false;
    primes[1] = false;
    
    for i in 2..n{
        if primes[i] == false{
            continue;
        }
        
        for j in 2..{
            let k = j * i;
            if k >= n{
                break;
            }
            primes[k] = false;
        }
    }
    
    let mut result = vec![];
    
    for i in 2..n{
        if primes[i] == false{
            continue;
        }
        
        if m == 1{
            break;
        }
        
        let mut cnt = 0;
        while m % i as i64 == 0{
            m /= i as i64;
            cnt+=1;
        }
        
        if cnt != 0{
            result.push((i as i64,cnt));
        }
    }
    
    result
}

fn power(mut x:i64,mut n:i64)->i64{
    let mut ret = 1;
    
    while n != 0{
        if n % 2 == 1{
            ret = (ret * x) % N_PRIME;
        }
        
        x = (x*x) % N_PRIME;
        n/=2;
    }
    
    ret
}

fn inverse_map(n_max:usize)->(Vec<i64>,Vec<i64>){
    let mut factorial = vec![0;n_max+1];
    let mut inverse_factorial = vec![0;n_max+1];
    
    factorial[0] = 1;
    for i in 1..n_max+1{
        factorial[i] = (i as i64 *factorial[i-1]) % N_PRIME;
    }
    
    inverse_factorial[n_max] = power(factorial[n_max],N_PRIME-2);
    for i in (0..n_max).rev(){
        inverse_factorial[i] = ((i as i64 +1)*inverse_factorial[i+1]) % N_PRIME;
    }
    
    (factorial,inverse_factorial)
}

fn main(){
    let inputs  = read_ints();
    let (n,m) = (inputs[0] as usize,inputs[1]);
    let (factorial,inverse_factorial) = inverse_map(100105);
    let factors = calc_factors(m);
    
    if n == 1{
        println!("{}",1);
        return;
    }
    
    /*
    println!("{}",{
            let a = cmp::max(6,2);
            let b = cmp::min(6,2);
            ((((factorial[a as usize]*inverse_factorial[b as usize]) % N_PRIME)*inverse_factorial[(a-b) as usize]) % N_PRIME)
        }
    );*/
    
    //println!("*******2");

    //println!("{:?}",factors);
    
    let mut ans = 1;
    
    for fact in factors{
        let (_,k) = fact;
        
        let temp = {
            let a = cmp::max(n as i64-1+k,k);
            let b = cmp::min(n as i64-1+k,k);
            let t = ((((factorial[a as usize]*inverse_factorial[b as usize]) % N_PRIME)*inverse_factorial[(a-b) as usize]) % N_PRIME);
            
            //println!("{} C {} = {}",a,b,t);
            t
        };
        
        ans = (ans*temp) % N_PRIME;
    }
    
    println!("{}",ans);
}
