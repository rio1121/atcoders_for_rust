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
    let n: i32 = read();
    let an: Vec<i32> = read_vec();
    let mut result: Vec<i32> = Vec::with_capacity(n as usize);

    for i in 0..n
    {
        result.push(0);
    }

    for i in 0..n
    {
        result[(an[i as usize] - 1) as usize] = i + 1;
    }

    for i in 0..n
    {
        print!("{}", result[i as usize]);
        if i < n - 1 { print!(" "); }
    }

    println!("");
}
