// アルゴリズムパズル 7. 真夜中の橋渡り
use std::io;
use std::cmp::{max,min};
use std::collections::BinaryHeap;

fn read_line() -> String{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn read_ints() -> Vec<i64>{
    let s = read_line();
    let split:Vec<&str> = s.split(" ").collect();
    split.iter().map(|&x| x.to_string().parse().unwrap()).collect()
}

fn take_2_pattern(xs:&Vec<i64>) -> Vec<(usize,usize)>{
    let mut ret = vec![];

    for i in 0..xs.len(){
        for j in i+1..xs.len(){
            ret.push((i,j));
        }
    }

    ret
}

fn solve(from:Vec<i64>,dest:BinaryHeap<i64>)->i64{
    if from.is_empty(){
        return 0;
    }else if from.len() <= 2{
        return max(from[0],from[1]);
    }

    let pattern = take_2_pattern(&from);
    let mut min_val = 9999999999;
    for (i,j) in pattern{
        let mut from = from.clone();
        // 小さい順のbinaryheapがわからないので符号を反転して使う(良くないが)
        let mut dest = dest.clone(); 
        let mut cost = max(from[i],from[j]);

        // i<jよりjを先に削除する
        dest.push(-from.remove(j));
        dest.push(-from.remove(i));

        // 返ってくるのは橋の向こう側にいるやつで一番早く返ってこれるやつでよいはず
        cost += -dest.peek().unwrap();
        from.push(-dest.pop().unwrap());

        min_val = min(min_val,cost + solve(from,dest));
    }

    min_val
}

fn main(){
    let xs = {
        let mut temp = read_ints();
        temp.sort();
        temp
    };

    println!("{}",solve(xs,BinaryHeap::new()));
}
