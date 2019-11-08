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
    let n_y: Vec<i32> = read_vec();
    let number_of_bills = n_y[0];
    let target_amount   = n_y[1];
    let mut result = [-1,-1,-1];

    'loop_of_10k: for bill_of_10k in 0..number_of_bills + 1
    {
        if bill_of_10k * 10000 > target_amount { break; }
        
        'loop_of_5k: for bill_of_5k in 0..number_of_bills + 1
        {
            if bill_of_5k * 5000 > target_amount { break; }
            if bill_of_10k + bill_of_5k > number_of_bills { break; }

            // 残りの枚数が千円札である
            let bill_of_1k = number_of_bills - (bill_of_10k + bill_of_5k);
            if (bill_of_10k * 10000) + (bill_of_5k * 5000) + (bill_of_1k * 1000) == target_amount
            {
                result = [bill_of_10k, bill_of_5k, bill_of_1k];
                break 'loop_of_10k;
            }
        }
    }

    println!("{} {} {}", result[0], result[1], result[2]);
}
