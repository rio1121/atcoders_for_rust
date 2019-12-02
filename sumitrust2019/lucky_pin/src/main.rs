// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let _n: u32 = read();
    let s: String = read();
    // 使用可能なPINの数
    let mut pin_count: u32 = 0;
    let num_str = vec!['0','1','2','3','4','5','6','7','8','9'];

    // 000-999の暗証番号について調べる.
    for digit100 in 0..10
    {
        for digit10 in 0..10
        {
            for digit1 in 0..10
            {
                let mut valid_digit = vec![false, false];

                for ch in s.chars()
                {
                    if !valid_digit[0] && !valid_digit[1] && ch == num_str[digit100] { valid_digit[0] = true; continue; }
                    if valid_digit[0] && !valid_digit[1] && ch == num_str[digit10] { valid_digit[1] = true; continue; }
                    // このPINは選ばれる
                    if valid_digit[0] && valid_digit[1] && ch == num_str[digit1] {
                        pin_count += 1;
                        break;
                    }
                }
            }
        }
    }

    // 結果を出力
    println!("{}", pin_count);
}
