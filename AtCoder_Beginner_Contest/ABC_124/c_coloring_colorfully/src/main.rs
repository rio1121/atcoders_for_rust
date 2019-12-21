// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let s: String = read();
    let mut svec: Vec<char> = s.chars().collect();
    let mut change_count = 0_u32;

    for i in 1..svec.len()
    {
        if svec[i] == svec[i - 1]
        {
            svec[i] = if svec[i] == '0' { '1' } else { '0' };
            change_count += 1;
        }
    }

    println!("{}", change_count);
}
