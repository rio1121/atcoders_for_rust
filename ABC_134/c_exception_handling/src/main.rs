// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

use std::cmp::*;

fn main() {
    let n: usize = read();
    let mut a: Vec<u64> = Vec::new();

    for _ in 0..n
    {
        let ai: u64 = read();
        a.push(ai);
    }

    let mut winner: u64 = max(a[0], a[1]);
    let mut runner_up: u64 = min(a[0], a[1]);
    let mut winner_index: u32 = if winner == a[0] { 0 } else { 1 };

    for i in 2..n
    {
        // 該当の値がwinner以上である場合
        if a[i] >= winner
        {
            runner_up = winner;
            winner = a[i];
            winner_index = i as u32;
            continue;
        }

        // 該当の値がwinner未満runner_up以上である場合
        if a[i] >= runner_up
        {
            runner_up = a[i];
        }
    }

    // winner_indexの時だけrunner_upの値を表示すればOK
    for i in 0..n
    {
        if i == winner_index as usize { println!("{}", runner_up); continue; }
        println!("{}", winner);
    }
}
