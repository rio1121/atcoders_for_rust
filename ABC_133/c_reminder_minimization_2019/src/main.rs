use std::cmp::*;

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
    let l_r: Vec<u64> = read_vec();
    let l = l_r[0];
    let r = l_r[1];

    // 調査A
    // lが0: 当然0が最小
    if l == 0 { println!("0"); return; }

    // 調査B
    // r - l == 1: その数同士の積しか数えられない
    if r - l == 1 { println!("{}", (r*l) % 2019); return; }

    // 調査C
    // l or rが673の倍数: 2019で割り切れるi, jの組み合わせが存在し、答えは0
    // 調査Bにおいて、673-674のような割り切れない組み合わせが除外されるため
    if l % 673 == 0 || r % 673 == 0 { println!("0"); return; }

    // 調査D
    // 2019 = 3 * 673
    // l / 673の整数部分とr / 673の整数部分が異なる
    if (l / 673) as u64 != (r / 673) as u64 {
        // 間に673の倍数が存在する
        // さらに、3の倍数が存在すれば2019で割り切れるi, jの組み合わせが存在し、答えは0
        if (l / 3) as u64 != (r / 3) as u64 {
            println!("0");
            return;
        }
    }

    // 調査E
    // ここまでに引っかからない場合は0にはできないので、最小の余りを求める
    let mut min_mod_result: u64 = 2018;
    for i in l..(r + 1)
    {
        for j in (i + 1)..(r + 1)
        {
            min_mod_result = min(min_mod_result, i*j%2019);
        }
    }

    println!("{}", min_mod_result);
}
