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

// 選択ソートを行う return: 交換回数
fn selection_sort(n: u32, a: &mut Vec<u32>) -> u32
{
    let mut exchange_count = 0;

    for i in 0..n as usize
    {
        // 範囲(j)内の最小値を持つ添字
        let mut min_j = i;
        for j in i..n as usize
        {
            if a[min_j] > a[j]
            {
                // 最小値を持つ添字を更新
                min_j = j;
            }
        }
        // 交換
        let tmp = a[i];
        a[i] = a[min_j];
        a[min_j] = tmp;
        // 最小値が更新された場合のみ
        if i != min_j { exchange_count += 1; }
    }

    return exchange_count;
}

fn main() {
    let n: u32 = read();
    let mut a: Vec<u32> = read_vec();

    let exchange_count = selection_sort(n, &mut a);
    // 現在の配列をスペース区切りで出力
    print_vec(a.to_vec());
    println!("{}", exchange_count);
}
