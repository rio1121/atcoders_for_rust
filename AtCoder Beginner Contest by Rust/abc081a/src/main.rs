// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let squares: String = read();
    let mut count_of_one = 0;
    // 前から1文字ずつ調べる
    for ch in squares.as_str().chars()
    {
        count_of_one += if ch == '1' { 1 } else { 0 };
    }

    println!("{}", count_of_one);
}
