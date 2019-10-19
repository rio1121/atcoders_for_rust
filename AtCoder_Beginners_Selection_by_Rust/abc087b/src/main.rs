// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let count_of_500:  i32 = read();
    let count_of_100:  i32 = read();
    let count_of_50:   i32 = read();
    let target_amount: i32 = read();

    let mut num_of_combinations: i32 = 0;

    'loop_of_500: for using_500 in 0..(count_of_500 + 1)
    {
        if using_500 * 500 > target_amount { break 'loop_of_500; }
        'loop_of_100: for using_100 in 0..(count_of_100 + 1)
        {
            if (using_500 * 500 + using_100 * 100) > target_amount { break 'loop_of_100; }
            let remaining_amount = target_amount - (using_500 * 500 + using_100 * 100);
            num_of_combinations += if count_of_50 * 50 >= remaining_amount { 1 } else { 0 }; 
        }
    }

    println!("{}", num_of_combinations);
}
