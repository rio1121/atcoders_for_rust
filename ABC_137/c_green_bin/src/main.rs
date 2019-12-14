use std::collections::*;

// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n: usize = read();
    let mut hashmap = HashMap::new();
    let mut combination_count: u64 = 0;

    for _ in 0..n as usize
    {
        let si: String = read();
        let mut si_chars: Vec<char> = si.chars().collect();
        si_chars.sort();
        let sorted_si: String = si_chars.into_iter().collect();

        // 初出のキーであればハッシュマップに挿入する
        let count = hashmap.entry(String::from(sorted_si)).or_insert(0);
        if *count > 0 {
            combination_count += *count;
            *count += 1;
        }

        if *count == 0 { *count = 1; }
    }

    println!("{}", combination_count);
}
