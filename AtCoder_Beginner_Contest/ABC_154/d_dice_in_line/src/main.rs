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

fn main() {
    let nk: Vec<usize> = read_vec();
    let n = nk[0];
    let k = nk[1];
    let p: Vec<f64> = read_vec();

    let mut max = 0.0_f64;

    // 期待値ベクター
    let mut e_vec = vec![0.0; n];
    let mut e_sum = 0.0_f64;

    for i in 0..n
    {
        let mut sum: f64 = 0.0;
        sum = (p[i] * (p[i] + 1.0)) / 2.0;

        e_vec[i] = sum / p[i];
        e_sum += e_vec[i];

        if i >= k
        {
            e_sum -= e_vec[i - k];
        }

        if e_sum > max { max = e_sum; }
    }

    println!("{}", max);
}
