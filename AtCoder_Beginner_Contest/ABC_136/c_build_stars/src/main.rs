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
    let h: Vec<i64> = read_vec();
    let mut is_continuation: bool = false;

    // 1度減少してから1度も増加せずに再び減少した場合はNo
    // 2以上の値減少した場合はNo
    for i in 0..(n - 1)
    {
        // 次の要素が1だけ少ない場合
        if h[i] - h[i + 1] == 1
        {
            // 増加することなく連続して発生した場合 - 強制終了
            if is_continuation { println!("No"); return; }

            // フラグをたてて続行
            is_continuation = true;
            continue;
        }

        // 次の要素が2以上少ない場合 - 強制終了
        if h[i] - h[i + 1] >= 2 { println!("No"); return; }

        // 増加した場合 - フラグ消去
        if h[i] - h[i + 1] < 0
        {
            is_continuation = false;
        }
    }

    println!("Yes");
}
