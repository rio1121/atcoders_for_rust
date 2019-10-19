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
    let number_of_integer: i32 = read();
    // ミュータブルでVecを作成
    let mut integers: Vec<i32> = read_vec();

    let mut count_of_operation: i32 = 0;
    // '[label]: loop　とすると多重ループを簡単に抜け出せる
    'main_loop: loop
    {
        /*
            配列の値を操作する:
            1: ミュータブルな配列を&mutをつけてin &mut [var]とする(参照)
            2: ループ内で各要素の値を参照する時は*をつける
        */
        for integer in &mut integers
        {
            if *integer % 2 == 1 { break 'main_loop; }
            *integer /= 2;
        }

        count_of_operation += 1;
    }

    println!("{}", count_of_operation);
}
