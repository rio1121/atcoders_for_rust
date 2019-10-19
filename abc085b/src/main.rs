// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let number_of_steps: i32 = read();
    let mut steps = Vec::new();

    for i in 0..number_of_steps
    {
        let value: i32 = read();
        steps.push(value);
    }

    // ソートして重複を削除する
    steps.sort();
    steps.dedup();

    println!("{}", steps.len());
}
