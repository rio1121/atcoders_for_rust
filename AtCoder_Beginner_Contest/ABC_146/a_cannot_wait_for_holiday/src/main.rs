// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s: String = read();
    match s.as_str(){
        "SUN" => println!("7"),
        "MON" => println!("6"),
        "TUE" => println!("5"),
        "WED" => println!("4"),
        "THU" => println!("3"),
        "FRI" => println!("2"),
        "SAT" => println!("1"),
        _ => println!("0"),
    }
}
