use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Kind{
    None,
    Cabbage,
    Wolf,
    Goat,
}

// 解法の表示
fn show_moves(moves:Vec<Kind>){
    println!("----- solved! -----");
    for m in moves {
        if m == Kind::None{
            println!("move nothing.");
        }else{
            println!("move {:?}",m);
        }
    }
    println!("-------------------");
}

// left_r,right_r,moves_rの状態で，mを動かした場合，
// 不正な操作でなければ次状態を返す関数
fn get_all_states(left_r:&HashSet<Kind>,right_r:&HashSet<Kind>,moves_r:&Vec<Kind>,m:Kind)
        ->Result<(HashSet<Kind>,HashSet<Kind>,Vec<Kind>),()>{
    // left->rightとして考える

    let to_right = moves_r.len() % 2 == 0;
    let mut left = if !to_right {right_r.clone()}else{left_r.clone()};
    let mut right = if to_right {right_r.clone()}else{left_r.clone()};
    
    if m != Kind::None{
        if !left.contains(&m){
            return Err(())
        }

        left.remove(&m);
        right.insert(m.clone());
    }

    // 捕食関係が保たれていたら駄目
    if (left.contains(&Kind::Cabbage) && left.contains(&Kind::Goat)) 
        ||  (left.contains(&Kind::Wolf) && left.contains(&Kind::Goat)){
        return Err(());
    }

    let mut moves = moves_r.clone();
    moves.push(m.clone());

    if to_right{
        Ok((left,right,moves))
    }else{
        Ok((right,left,moves))
    }
}

// 次状態を取得する
// ゲームが終了していた場合，解法を表示しNoneを返す
fn solve(left: HashSet<Kind>,right:HashSet<Kind>,moves:Vec<Kind>)
    ->Option<Vec<(HashSet<Kind>,HashSet<Kind>,Vec<Kind>)>>{

    if left.len() == 0{
        // 左岸になにもない→成功
        show_moves(moves);
        None
    }else{
        let mut ret = vec![];       // 次状態

        for m in vec![Kind::Wolf,Kind::Cabbage,Kind::Goat,Kind::None]{
            if let Ok(v) = get_all_states(&left,&right,&moves,m){
                ret.push(v);
            }
        }

        Some(ret)
    }
}

// 幅優先探索で解く
fn solves(mut xs:Vec<(HashSet<Kind>,HashSet<Kind>,Vec<Kind>)>){
    let mut min_hand = 9999999;

    loop{
        let (left,right,moves) = xs.remove(0);
        let n_hand = moves.len();

        if n_hand > min_hand{
            break;
        }

        if let Some(mut v) = solve(left,right,moves){
            xs.append(&mut v);
        }else{
            min_hand = n_hand;
        }
    }
}

fn main(){
    let left = {
        let mut h = HashSet::new();
        for v in vec![Kind::Wolf,Kind::Cabbage,Kind::Goat]{
            h.insert(v);
        }
        h
    };
    let right = HashSet::new();

    solves(vec![(left,right,vec![])]);
}