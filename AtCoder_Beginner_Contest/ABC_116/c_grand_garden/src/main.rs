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

// 1以上の要素が何個の区間に分割されるかを返す.
fn section_divison(v: &Vec<i64>) -> u64 {
    let mut i = 0_usize;
    let mut division_num = 0_u64;

    while i < v.len() {
        // 要素が0以下であれば何もしない
        if v[i] <= 0 { i += 1; continue; }

        // 区間開始
        division_num += 1;
        while i < v.len() && v[i] > 0 { i += 1; }
    }

    division_num
}

fn main() {
    let _n: usize = read();
    let mut h: Vec<i64> = read_vec();

    let mut count  = 0_u64;
    loop {
        // 全要素が0なら終了
        let max_value = h.iter().max().unwrap();
        if *max_value <= 0 { break; }

        // 区間分割
        count += section_divison(&h);

        // 1減らす
        h = h.iter().map(|x| x - 1).collect();
    }

    println!("{}", count);
}
