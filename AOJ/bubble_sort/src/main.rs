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

// 配列要素をスペースで区切って出力する。
fn print_vec(v: Vec<u32>)
{
    let mut is_first: bool = true;

    for element in v.iter()
    {
        if !is_first { print!(" "); }
        print!("{}", element);
        is_first = false;
    }

    println!("");
}

// バブルソートを行う return: ソートに要した交換回数
// 実際にはフラグ変数等を用いて、内側のループ中に1度も交換が発生しなければ打ち切ってしまって良い
fn bubble_sort(n: u32, a: &mut Vec<u32>) -> u32
{
    let mut exchange_count = 0;
    // 降順に参照するループ
    for i in 1..n as usize
    {
        for j in 1..n as usize
        {
            if a[j - 1] > a[j]
            {
                // 交換
                let tmp = a[j];
                a[j] = a[j - 1];
                a[j - 1] = tmp;
                exchange_count += 1;
            }
        }
    }

    return exchange_count;
}

fn main() {
    let n: u32 = read();
    let mut a: Vec<u32> = read_vec();

    let exchange_count = bubble_sort(n, &mut a);
    // 現在の配列をスペース区切りで出力
    print_vec(a.to_vec());
    println!("{}", exchange_count);
}
