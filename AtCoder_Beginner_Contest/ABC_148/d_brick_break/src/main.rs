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
    let a: Vec<u64> = read_vec();

    // 対象の数を決める。1から見つかる度にインクリメント。
    let mut search_number = 1_u64;
    let mut break_count = 0;

    for i in 0..n
    {
        // 対象の数である場合
        if a[i] == search_number
        {
            // このレンガは砕かなくて良い
            search_number += 1;
            continue;
        }

        break_count += 1;
    }

    if search_number == 1 { println!("-1"); }
    else { println!("{}", break_count); }
}
