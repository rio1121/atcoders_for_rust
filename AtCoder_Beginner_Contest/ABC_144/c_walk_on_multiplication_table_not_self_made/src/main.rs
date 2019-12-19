// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let number: u64 = read();
    // 初期の最小値として非常に大きな値を指定する
    let mut min_number_of_trials: u64 = 10u64.pow(12);

    // 探索範囲は対称性からsqrt(N)までで良い。
    for i in 1..(number as f64).sqrt() as u64 + 1
    {
        // 割り切れる値を発見したら、試行回数が最小値であるかどうか調べ、最小値であればアップデートする。
        if number % i == 0
        {
            let now_number_of_trials = (number / i) + i - 2;
            if now_number_of_trials < min_number_of_trials
            {
                min_number_of_trials = now_number_of_trials;
            }
        }
    }

    println!("{}", min_number_of_trials);
}
