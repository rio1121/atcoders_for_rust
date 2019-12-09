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

const MOD: u64 = 1e9 as u64 + 7;

fn main() {
    let n: u32 = read();
    let a: Vec<u64> = read_vec();
    let mut sum: u64 = 0;

    for i in 0..60 as usize
    {
        // 最下位ビットが1であるAの数
        let mut ones = 0;
        // 最下位ビットが0であるAの数
        let mut zeros = 0;
        for j in 0..n as usize
        {
            if (a[j] >> i) & 1 == 1 { ones += 1; } else { zeros += 1; }
        }

        let per = (1u64 << i) % MOD;
        sum += ones * zeros % MOD * per % MOD;
        sum %= MOD;
    }

    println!("{}", sum);
}
