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

// 数値nの桁数を返す
fn digit(n: u64) -> u64 {
    let stringed_number = n.to_string();
    return stringed_number.len() as u64;
}

// 整数nは購入可能か
fn is_available_for_purchase(a: u64, b: u64, x: u64, n: u64) -> bool
{
    if a * n + b * digit(n) <= x { return true; } else { return false; }
}

fn main() {
    let a_b_x: Vec<u64> = read_vec();
    let a = a_b_x[0];
    let b = a_b_x[1];
    let x = a_b_x[2];

    let mut n_max: u64 = 10_u64.pow(9);
    let mut n_min: u64 = 0;
    // 整数nが購入可能か
    if is_available_for_purchase(a, b, x, n_max)
    {
        println!("{}", n_max);
        return;
    }

    let mut answer = 0;
    let mut check_value = (n_max + n_min) / 2;

    loop
    {
        // 整数nが購入可能か
        if is_available_for_purchase(a, b, x, check_value)
        {
            answer = check_value;
            // 下の値を更新
            n_min = check_value;
            check_value = (n_max + n_min) / 2;

            if check_value == n_min { break; }
        }
        else
        {
            // 上の値を更新
            n_max = check_value;
            check_value /= 2;

            if check_value == n_max { break; }
        }

        
    }

    println!("{}", answer);
}
