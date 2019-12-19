// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
    
fn main() {
    let number: i32 = read();
    
    // 1-9の整数であれば、無条件で正
    if (number < 10 && number > 0)
    {
        println!("Yes");
        return;
    }
    
    // 1-以上の値に関しては、
    // ・2-9の値で割り切れること
    // ・2-9の値で割った商が9以下であること
    // が満たされている場合に正とする。
    for i in 2..10
    {
        if (number % i == 0 && number / i <= 9)
        {
        println!("Yes");
        return;
        }
    }
    
    println!("No");
}