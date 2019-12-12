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
    let n: u32 = read();
    let a: Vec<u32> = read_vec();
    let q: u32 = read();
    let m: Vec<u32> = read_vec();
    let mut all_sum: Vec<u32> = Vec::new();

    // 合計値の配列を作る
    for bits in 0..(1 << n)
    {
        let mut sum = 0_u32;

        for index in 0..n
        {
            if bits >> index & 1 == 1 { sum += a[index as usize]; }
        }

        all_sum.push(sum);
    }

    all_sum.sort();

    // 各mについて調べる.
    for mi in m
    {
        // 二分探索
        if all_sum.binary_search(&mi).is_ok() { println!("yes"); } else { println!("no"); } 
    }
}
