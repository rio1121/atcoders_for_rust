// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
    
fn main() {
    let n: i32      = read();
    let s: String   = read();

    if s.len() % 2 != 0 { 
        println!("No");
        return; 
    }

    let front = &s[0..(s.len()/2)];
    let back = &s[(s.len()/2)..s.len()];

    if front == back { println!("Yes"); } else { println!("No"); }
}
