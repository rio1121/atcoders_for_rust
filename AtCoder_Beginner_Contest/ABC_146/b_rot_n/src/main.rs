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

fn advance(c: char, n: i32) -> char {
    let alphabet_index: u8 = c as u8 - 65;
    let converted_index: i32 = (alphabet_index as i32 + n) % 26;
    return (converted_index + 65) as u8 as char;
}

fn main() {
    let n: i32 = read();
    let mut s: String = read();
    let mut result: String = "".to_string();

    for c in s.chars()
    {
        result.push(advance(c, n));
        
    }

    println!("{}", result);
}
