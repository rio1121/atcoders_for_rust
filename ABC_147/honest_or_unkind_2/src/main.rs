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

use std::cmp::*;

fn main() {
    let n: usize = read();
    // 証言の配列
    let mut testimonies= vec![vec![]; n];
    for index in 0..n
    {
        // 各人の証言を入力
        let a_n: usize = read();
        for _ in 0..a_n
        {
            let testimony: Vec<u32> = read_vec();
            // タプルを突っ込む: .0=対象の人 .1=正直者か不親切な人か
            testimonies[index].push((testimony[0] - 1, testimony[1]));
        }
    }

    // 正直者の最大人数
    let mut honest_count_max: u32 = 0;
    // 証言の検証 bit全探索
    'outer: for bp in 0..1 << n
    {
        // 正直者である人物の証言を調べ、矛盾を確かめる.
        for index in 0..n
        {
            // 正直者である場合、証言を調べる
            if bp >> index & 1 == 1
            {
                for t_index in 0..testimonies[index].len()
                {
                    // 矛盾の発見
                    if bp >> testimonies[index][t_index].0 & 1 != testimonies[index][t_index].1
                    {
                        continue 'outer;
                    }
                }
            }
        }

        // count_ones()はビット列中の'1'の数を返す
        honest_count_max = max(honest_count_max, bp.count_ones());
    }

    println!("{}", honest_count_max);
}
