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
    let nq: Vec<usize> = read_vec();
    let n = nq[0];
    let q = nq[1];
    let s: String = read();

    // i + 1文字目までに現れた'AC'の数を事前に数える
    let mut appear_count = vec![0; n];
    let svec: Vec<char> = s.chars().collect();
    for i in 1..n
    {
        if svec[i - 1] == 'A' && svec[i] == 'C' { appear_count[i] += 1; }
        appear_count[i] += appear_count[i - 1];
    }

    // 各問題を作ったappear_count配列を使って解く。
    for _ in 0..q
    {
        let lr: Vec<usize> = read_vec();
        let l = lr[0];
        let r = lr[1];
        println!("{}", appear_count[r - 1] - appear_count[l - 1]);
    }
}
