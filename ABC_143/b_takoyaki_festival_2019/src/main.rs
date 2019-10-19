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

fn main() {
    let number: i32 = read();
    let mut takoyaki: Vec<i32> = read_vec();

    let mut recovery_value: i32 = 0;

    loop
    {
        let parent_number = takoyaki[0];
        takoyaki.remove(0);
        if takoyaki.len() <= 0 { break; }

        for value_of_takoyaki in &takoyaki
        {
            recovery_value += value_of_takoyaki * parent_number;
        }
    }

    println!("{}", recovery_value);
}
