// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let mut h: i64 = read();
    let mut count = 0_u32;

    // 2で割っていく
    loop {
        if h == 1 { break; }

        h /= 2;
        count += 1;
    }

    let mut attacks: u64 = 0;
    for i in 0..(count + 1) as usize {
        attacks += 2_u64.pow(i as u32);
    }
    println!("{}", attacks);
}
