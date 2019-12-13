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
    let n_k_q: Vec<u64> = read_vec();
    let mut a: Vec<usize> = Vec::new();
    let mut point: Vec<u64> = vec![0; n_k_q[0] as usize];

    for _ in 0..n_k_q[2] as usize
    {
        let ai: usize = read();
        a.push(ai);
    }

    for ai in a
    {
        // 正解数を加算
        point[ai - 1] += 1;
    }

    for p in point
    {
        if (n_k_q[2] - p) < n_k_q[1] { println!("Yes"); } else { println!("No"); }
    }
}
