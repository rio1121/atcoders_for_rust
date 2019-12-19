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
    let n = nm[0];
    let m = nm[1];
    // 対象となるスイッチを表すbit列
    // ex. スイッチが5個で、i個目の電球に紐づいたスイッチが1,2,5
    // => 10011
    let mut bits: Vec<u64> = vec![0; m];

    for i in 0..m
    {
        let km: Vec<u32> = read_vec();
        for j in 0..km[0] as usize
        {
            bits[i] += 1 << (km[1 + j] - 1);
        }
    }

    let p: Vec<u32> = read_vec();
    let mut result: u64 = 0;

    // n個のスイッチに関してbit全探索
    'outer: for bp in 0..1 << n
    {
        // 各電球が点灯するか調べる, 点灯しないパターンが存在する場合はcontinue
        for i in 0..m
        {
            // AND
            let and = bits[i] & bp;
            if and.count_ones() % 2 != p[i] { continue 'outer; }
        }

        result += 1;
    }

    println!("{}", result);
}
