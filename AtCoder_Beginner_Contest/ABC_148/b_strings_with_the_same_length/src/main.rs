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
    let n: usize = read();
    let st: Vec<String> = read_vec();
    let s = &st[0];
    let t = &st[1];

    let svec: Vec<char> = s.chars().collect();
    let tvec: Vec<char> = t.chars().collect();

    let mut stvec: Vec<char> = Vec::new();

    for i in 0..n
    {
        stvec.push(svec[i]);
        stvec.push(tvec[i]);
    }

    let result: String = stvec.into_iter().collect();

    println!("{}", result);
}
