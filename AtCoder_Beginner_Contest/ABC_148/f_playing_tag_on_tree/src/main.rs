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

fn search(prev: usize, data: &Vec<Vec<usize>>, tkhs: usize, aoki: usize) -> u64
{
    // 前のマスに戻るしかない場合は終了
    if prev != 0 && prev == data[tkhs][0];
}

fn main() {
    let nuv: Vec<u64> = read_vec();
    let n = nuv[0];
    let mut u = nuv[1];
    let mut v = nuv[2];

    let mut aoki_move_count = 0_u64;

    // [0]はダミー
    let mut root_data: Vec<Vec<usize>> = vec![Vec::new(); n as usize + 1];

    for i in 0..(n as usize - 1)
    {
        let ab_tmp: Vec<usize> = read_vec();
        root_data[ab_tmp[0]].push(ab_tmp[1]);
        root_data[ab_tmp[1]].push(ab_tmp[0]);
    }

    loop
    {
        // 高橋君の試行: 選択肢を取得
        let selection = root_data[u as usize];
        for i in selection
        {
            // 各選択肢について、青木君の場所にたどり着けるか調べる
            let count = search(0, &root_data, u as usize, v as usize);
        }
    }

    println!("{:?}", root_data);
}
