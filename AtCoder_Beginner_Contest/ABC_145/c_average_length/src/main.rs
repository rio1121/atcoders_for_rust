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

// 標準入力からn行を読み取り、各行を空白文字で分割し、各要素を指定の型に変換する関数
fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

// 階乗
fn factorial(n: i32) -> i32 {
    let mut factorial_value: i32 = 1;
    for i in 1..(n + 1)
    {
        factorial_value *= i;
    }

    return factorial_value;
}
    
fn main() {
    let n: i32 = read();
    let xy: Vec<Vec<i32>> = read_vec2(n as u32);
    let mut sum_distance: f64 = 0.0;
    let mut distance_count: i32 = 0;

    // 経路はN!通りだが、実際に必要な町間距離の情報は(1..N の和)通りである。
    for i in 0..(n - 1) as usize
    {
        for j in (i + 1)..n as usize
        {
            let distance: f64 = (( (xy[i][0] - xy[j][0]).pow(2) + (xy[i][1] - xy[j][1]).pow(2) ) as f64).sqrt();
            sum_distance += distance;
            distance_count += 1;
        }
    }

    // 町間距離の総計は(N - 1) * N!を取得した町間距離で割った数
    sum_distance *= ((n - 1) * factorial(n) / distance_count) as f64;
    // N!で割る
    sum_distance /= factorial(n) as f64;

    println!("{}", sum_distance);
}
