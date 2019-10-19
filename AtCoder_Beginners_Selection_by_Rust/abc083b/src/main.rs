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

fn main() {
    let input_numbers: Vec<i32> = read_vec();
    let mut total_value: i32 = 0;

    for i in 1..(input_numbers[0] + 1)
    {
        let mut sum_each_digit = 0;
        let mut tmp_i = i; // iの値を変更する操作を行うため退避させる
        loop
        {
            sum_each_digit += tmp_i % 10;
            if tmp_i < 10 { break; } else { tmp_i /= 10; }
        }

        total_value += if (sum_each_digit >= input_numbers[1]) && (sum_each_digit <= input_numbers[2]) { i } else { 0 };
    }

    println!("{}", total_value);
}
