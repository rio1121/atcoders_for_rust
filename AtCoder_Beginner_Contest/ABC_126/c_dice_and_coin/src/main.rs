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

// kの値を超えるために必要なx2乗算の回数を返す
fn count_2x_multiplication(v: u64, k: u64) -> u64 {
    let mut vmut = v;
    let mut count = 1_u64;

    loop {
        vmut *= 2;
        if vmut >= k { break; }

        count += 1;
    }

    return count;
}

fn main() {
    let nk: Vec<u64> = read_vec();
    let n = nk[0] + 1_u64;
    let k = nk[1];
    let mut p: f64 = 0.0;
    let n_inv = 1.0 / nk[0] as f64; // 出目の確率

    for i in 1..n
    {
        // そもそも出目がk以上である場合
        if i >= k {
            p += n_inv;
            continue;
        }

        // k未満の出目の場合はコイン試行
        let count = count_2x_multiplication(i, k);
        p += n_inv * 0.5_f64.powf(count as f64);
    }

    println!("{}", p);
}
