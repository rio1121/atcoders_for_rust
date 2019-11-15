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
    let ab: Vec<u64> = read_vec();
    let mut a: u64 = 0;
    let mut b: u64 = 0;

    // 小さい方をAで固定する
    if ab[0] > ab[1]
    {
        a = ab[1];
        b = ab[0];
    }
    else
    {
        a = ab[0];
        b = ab[1];
    }

    let sqrt_a = ((a as f64).sqrt()) as usize + 1;
    let sqrt_b = ((b as f64).sqrt()) as usize + 1;

    let mut p_factor_a: Vec<u64> = Vec::with_capacity(sqrt_a);
    let mut p_factor_b: Vec<u64> = Vec::with_capacity(sqrt_b);

    // 1を追加する
    p_factor_a.push(1);
    p_factor_b.push(1);

    // Aの試し割り
    let mut i: u64 = 2;
    let mut is_added = false;
    loop
    {
        // 割り切れるなら素因数vecに追加し、iで割った値で次の試行へ進む
        if a % i == 0 
        {
            // 既に追加した場合はpushしない
            if !is_added 
            {
                p_factor_a.push(i);
                is_added = true;
            }
            a /= i;
            continue;
        }
        // 割り切れない場合はiをインクリメントする
        i += 1;
        is_added = false;
        // iがaの平方根を切り上げた整数値を超えたら終了。現在の値を加える。
        if i > sqrt_a as u64 
        {
            if a != 1 { p_factor_a.push(a); }
            break;
        }
    }

    // Bの試し割り
    i = 2;
    loop
    {
        // 割り切れるなら素因数vecに追加し、iで割った値で次の試行へ進む
        if b % i == 0 
        {
            // 既に追加した場合はpushしない
            if !is_added 
            {
                p_factor_b.push(i);
                is_added = true;
            }
            b /= i;
            continue;
        }
        // 割り切れない場合はiをインクリメントする
        i += 1;
        is_added = false;
        // iがbの平方根を切り上げた整数値を超えたら終了。現在の値を加える。
        if i > sqrt_b as u64 
        {
            if b != 1 { p_factor_b.push(b); }
            break;
        }
    }

    // Rustには配列のANDを取るメソッドが無いっぽいので強引な手法
    // (1) 予めAとBの素因数配列の要素数の和sを取る
    // (2) AにBの要素を追加する
    // (3) 重複を取り除く(重複除去後の配列の要素数をs'とする)
    // (4) sからs'を引いた値(削除した重複要素数のこと)が回答になる。
    let previous_sum_len = p_factor_a.len() + p_factor_b.len();
    p_factor_a.append(&mut p_factor_b);
    p_factor_a.sort();
    p_factor_a.dedup();

    println!("{}", previous_sum_len - p_factor_a.len());
}
