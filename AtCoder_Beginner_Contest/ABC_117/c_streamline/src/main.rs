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
    let nm: Vec<usize> = read_vec();
    let mut xm: Vec<i64> = read_vec();

    // N >= Mなら終了
    if nm[0] >= nm[1] { println!("{}", 0); return; }

    xm.sort();

    let mut distance: Vec<i64> = Vec::new();

    // 座標間の移動距離を求める
    for i in 0..(nm[1] - 1)
    {
        let d: i64 = (xm[i + 1] - xm[i]).abs();
        distance.push(d);
    }

    // 座標間の移動距離を小さい方からresultに加算する。
    // 加算回数はM - N回
    distance.sort();

    let mut result = 0_i64;
    for i in 0..(nm[1] - nm[0])
    {
        result += distance[i];
    }
    println!("{}", result);
}
