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

// 標準入力からn行を読み取り、各行を空白文字で分割し、各要素を指定の型に変換する関数
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

use std::cmp::*;

fn main() {
    let hn: Vec<i64> = read_vec();
    let abi: Vec<Vec<i64>> = read_vec2(hn[1] as u32);

    let h = hn[0];
    let max_cost = 10000 * 10000 as usize + 1;

    // 添字...消費コスト 値...与ダメージ
    let mut dp = vec![0_u64; max_cost];
    for i in 1..max_cost as usize {
        // 各魔法を使用した場合について考える
        for j in 0..hn[1] as usize {
            // 現在参照しているコストが魔法のコストより小さい場合はスルー
            if abi[j][1] > i as i64 { continue; }

            // 魔法jを使用した時に、残りのコストで与えられるダメージの最大値をDP配列から取得
            let short: usize = i - abi[j][1] as usize;

            // dpの更新
            dp[i] = max(dp[i], abi[j][0] as u64 + dp[short]);
        }

        // 現在のコストで与えられる最大の与ダメージがHP以上の値であれば終了.
        if dp[i] >= h as u64 { println!("{}", i); return; }
    }
}
