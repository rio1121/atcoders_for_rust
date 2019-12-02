// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let x: usize = read();

    // 動的計画法
    let mut dp = vec![false; 100001];
    // X = 0はtrue
    dp[0] = true;
    // X = 100-104はtrue
    dp[100] = true;
    dp[101] = true;
    dp[102] = true;
    dp[103] = true;
    dp[104] = true;

    // iから100~105の値を引いた添字の真偽値をチェックし、1つでもtrueが存在すればその金額は有効である。
    for i in 105..dp.len() as usize
    {
        if dp[i - 105] || dp[i - 104] || dp[i - 103] || dp[i - 102] || dp[i - 101] || dp[i - 100] { dp[i] = true; }
    }

    // 結果を出力
    if dp[x] { println!("1"); } else { println!("0"); }
}
