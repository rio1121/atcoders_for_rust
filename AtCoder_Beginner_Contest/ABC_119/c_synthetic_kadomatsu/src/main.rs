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

// 4つの値の最小値を求める
fn min4(a: i64, b: i64, c: i64, d: i64) -> i64 {
    let mut vec = vec![a, b, c, d];
    vec.sort();

    return vec[0];
}

// 再帰関数(深さ優先探索)
fn dfs(nabc: &Vec<i64>, l: &Vec<i64>, i: usize, a: i64, b: i64, c: i64) -> i64 {
    // 最後の竹を選択済み
    if i == nabc[0] as usize {
        // 一本も選択しないパターンがABCそれぞれに存在する組み合わせは極端に大きな値を返す(除外する)
        if a == 0 || b == 0 || c == 0 { return 10_i64.pow(9); }
        // 絶対値の和がコストになる。後述する理由により、ABCのコストの和から30を差し引いた値を返す.
        return (a - nabc[1]).abs() + (b - nabc[2]).abs() + (c - nabc[3]).abs() - 30;
    }

    // 使用しない場合: ret0
    // 合成した場合: ret1-3
    // 1本目でも合成扱いになるため、最終的にこの分は引いておかなければならない(上で-30する理由).
    let ret0 = dfs(&nabc, &l, i + 1, a, b, c);
    let ret1 = dfs(&nabc, &l, i + 1, a + l[i], b, c) + 10;
    let ret2 = dfs(&nabc, &l, i + 1, a, b + l[i], c) + 10;
    let ret3 = dfs(&nabc, &l, i + 1, a, b, c + l[i]) + 10;

    // 最終的に最も低いコストで収まるパターンを返す.
    return min4(ret0, ret1, ret2, ret3);
}

fn main() {
    let nabc: Vec<i64> = read_vec();
    let mut l: Vec<i64> = Vec::new();

    for _ in 0..nabc[0] as usize
    {
        let li: i64 = read();
        l.push(li);
    }

    // 全探索
    // 各竹には、Aの素材、Bの素材、Cの素材、使わないの4パターン存在するから、これを再帰関数で全探索する.
    // 添字0、ABC全て未選択の状態でスタート
    println!("{}", dfs(&nabc, &l, 0, 0, 0, 0));
}
