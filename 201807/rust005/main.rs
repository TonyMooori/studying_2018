// アルゴリズムパズル 5. 行と列の入れ替え

use std::io;
use std::process;

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

fn read_int() -> i64{
    read_ints()[0]
}

///////////////////////////////////////////////////

// xs = [3 1 2 4]というようなリストをtarget=[1 2 3 4]とおなじようにする置換のリストを返す
fn generate_permutations(mut xs:Vec<i64>,target:Vec<i64>)->Vec<(usize,usize)>{
    // とりあえず1から順に揃えるだけ
    let mut result = vec![];
    let n = xs.len();

    for i in 0..n{
        if target[i] != xs[i]{
            let j = (0..n)
                .find(|&j| xs[j] == target[i])
                .unwrap();
            result.push((i,j));
            xs.swap(i,j);
        }
    }

    result
}

// 行列を表示する
fn display_matrix(matrix:&Vec<Vec<i64>>){
    for i in 0..matrix.len(){
        println!("{:?}",matrix[i]);
    }
}

// matrixのperm.0列目とperm.1列目を入れかえる
fn swap_col(matrix:&mut Vec<Vec<i64>>,perm:(usize,usize),show:bool){

    for i in 0..matrix.len(){
        matrix[i].swap(perm.0,perm.1);
    }

    if show{
        println!("swap {} th column and {} th column!",perm.0+1,perm.1+1);
        display_matrix(&matrix);
    }
}

// matrixのperm.0行目とperm.1行目を入れかえる
fn swap_row(matrix:&mut Vec<Vec<i64>>,perm:(usize,usize),show:bool){

    for _ in 0..matrix.len(){
        matrix.swap(perm.0,perm.1);
    }
    
    if show{
        println!("swap {} th row and {} th row!",perm.0+1,perm.1+1);
        display_matrix(&matrix);
    }
}

// matrixの行からソートすると[1 2 3 4]となるような行を取り出す
fn fetch_row_1(matrix:&Vec<Vec<i64>>)->Option<Vec<i64>>{
    let target = (1..matrix.len() as i64 + 1).collect::<Vec<_>>();

    for row in matrix.iter(){
        // ソートした行の取り出し
        let mut row_dash = row.clone();
        row_dash.sort_unstable();

        if row_dash == target{
            return Some(row.clone())
        }
    }

    None
}

// matrixの列からソートすると[1 5 9 13]となるような列を取り出す
fn fetch_col_1(matrix:&Vec<Vec<i64>>)->Option<Vec<i64>>{
    let n = matrix.len();
    let target = (0..n).map(|i| (n * i + 1)as i64).collect::<Vec<_>>();

    for i in 0..n{
        let col = {
            let mut temp = vec![];
            for j in 0..n{
                temp.push(matrix[j][i])
            }
            temp
        };
        // ソートした列の取り出し
        let mut col_dash = col.clone();
        col_dash.sort_unstable();

        if col_dash == target{
            return Some(col)
        }
    }

    None
}

// matrixの列を入れ替えて揃える(左から右へ昇順になるようにする)置換のリストを返す
fn arrange_col(matrix:&Vec<Vec<i64>>)->Vec<(usize,usize)>{
    // ソートすると[1 2 3 4]となる行を取り出す
    let row1 = fetch_row_1(&matrix).unwrap_or_else(|| {
        println!("Invalid matrix.");
        process::exit(0);
    });
    let n = row1.len();

    // row1を[1 2 3 4]とするような列に対する置換のリストを生成
    generate_permutations(
        row1,
        (1..1+n as i64).collect::<Vec<_>>())
}


// matrixの各列の順番を揃える(上から下へ昇順になるようにする)置換のリストを返す
fn arrange_row(matrix:&Vec<Vec<i64>>)->Vec<(usize,usize)>{
    // ソートすると[1 5 9 13]となる列を取り出す
    let col1 = fetch_col_1(&matrix).unwrap_or_else(||{
        println!("Invalid matrix.");
        process::exit(0);
    });
    let n = col1.len();

    // col1を[1 2 3 4]とするような行に対する置換のリストを生成
    generate_permutations(
        col1,
        (0..n).map(|i| (n * i + 1)as i64).collect::<Vec<_>>())
}

// マトリックスが最終状態となっているか確認する関数
fn check_matrix(matrix:&Vec<Vec<i64>>) -> bool{
    let n = matrix.len();
    for i in 0..n{
        for j in 0..n{
            if matrix[i][j] != (1+i*n+j) as i64{
                return false;
            }
        }
    }
    return true;
}

// 行列を規定の形に行と列だけを入れ替えて揃えられるかを調べる関数
fn arrange_matrix(mut matrix:Vec<Vec<i64>>,show:bool){
    let perm_row = arrange_row(&matrix);
    let perm_col = arrange_col(&matrix);
    
    for perm in perm_col{
        swap_col(&mut matrix,perm,show);
    }
    for perm in perm_row{
        swap_row(&mut matrix,perm,show);
    }

    if check_matrix(&matrix){
        println!("Valid matrix.");
    }else{
        println!("Invalid matrix.");
    }
}

fn main(){
    // 入力の読み取り
    let n = read_int() as usize;
    let matrix = {
        let mut temp = vec![];
        for _ in 0..n{
            temp.push(read_ints());
        }
        temp
    };

    arrange_matrix(matrix,true);
}

// 入力を次のようにする
// 1行目……n(行列の次数)
// 2～n+1行目……行列
// 3
// 1 3 2
// 4 6 5
// 7 9 8
/*
3
7 9 8
1 3 2
4 6 5
-> Valid matrix.
3
7 9 8
1 3 2
4 5 6 
-> Invalid matrix.
4
12 10 11 9
16 14 5 13
8 6 7 15
4 2 3 1
*/