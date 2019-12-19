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

const MOD: u64 = 1_000_000_007;

fn main() {
    let nm: Vec<usize> = read_vec();
    let mut a: Vec<u64> = Vec::new();

    for i in 0..nm[1]
    {
        let ai: u64 = read();
        a.push(ai);
    }

    // 動的計画法
    let mut dp = vec![0_u64; nm[0] + 1];
    dp[0] = 1;

    // 崖崩れフラグ用変数
    let mut collapsed = 0_usize;

    for _ in 1..dp.len() as usize
    {
        if collapsed < nm[1] && a[collapsed] == (i as u64)
        {
            // 崩壊している場合、この足場にたどり着くパターンは存在しない
            dp[i] = 0;
            collapsed += 1;
            continue;
        }

        if i == 1 { dp[1] = 1; continue; }

        dp[i] = (dp[i - 1] + dp[i - 2]) % MOD;
    }
    println!("{}", dp[nm[0]]);
}
