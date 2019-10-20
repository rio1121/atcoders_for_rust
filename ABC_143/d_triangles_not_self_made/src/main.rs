// 解説動画視聴後に解答

// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
    
// 標準入力から一行を読み取り、空白文字で分割し、各要素を指定の型に変換する関数
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

// 配列vの添字startから探索し、keyの値以上の要素を見つけたらその要素を返す。
// 見つからなかった場合は、配列の最大添字の1つ上の値を返す
// 所有権が移る操作なので、Vec<i32>には&をつける。呼び出し側も同様。
fn lower_bound(v: &Vec<i32>, start: usize, key: i32) -> i32 {
    let mut result_index: i32 = v.len() as i32;

    for i in start..v.len()
    {
        if v[i] >= key
        {
            result_index = i as i32;
            break;
        }
    }

    result_index
}
    
fn main() {
    let number: i32 = read();
    let mut sticks: Vec<i32> = read_vec();
    
    let mut count_of_triangles: i32 = 0;

    // ソート
    sticks.sort();
    
    //a < b < cのとき、a + b > c
    // 最後の要素はbとして取り得ない
    for b in 0..(number - 1) as usize 
    {
        // a < bが成り立つので、aの取りうる添字は0からb - 1まで
        for a in 0..b as usize 
        {
            // 配列の要素を添字で取得する時はusize型を使う
            let ab: i32 = sticks[a] + sticks[b];
            let end_index: i32 = lower_bound(&sticks, b, ab);
            let begin_index: i32 = (b + 1) as i32;
            count_of_triangles += end_index - begin_index;
        }
    }
    
    println!("{}", count_of_triangles);
}
