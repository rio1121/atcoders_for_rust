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

// 挿入ソートを行う
fn insertion_sort(n: u32, a: &mut Vec<u32>)
{
    // 最初の状態を表示する
    // aは&mut Vec<u32>なので、関数の引数に合わせてVec<u32>にするためにto_vec()を使う
    print_vec(a.to_vec());

    for i in 1..n as usize
    {
        let value = a[i];
        // マイナスの値になる場合があるので、jはi32で使う
        let mut j: i32 = i as i32 - 1;
        while a[j as usize] > value
        {
            a[j as usize + 1] = a[j as usize];
            j -= 1;
            if j < 0 { break; }
        }
        // j + 1を計算してからusizeに戻す
        a[(j + 1) as usize] = value;

        // 現在の配列をスペース区切りで出力
        print_vec(a.to_vec());
    }
}

fn main() {
    let n: u32 = read();
    let mut a: Vec<u32> = read_vec();

    insertion_sort(n, &mut a)
}
