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

fn get_allocation(p: u32, k: u32, w: &Vec<Vec<u32>>) -> bool {
    let mut sum_weight: u32 = 0;
    let mut pack: usize = 0;

    'outer: for _ in 0..k as usize
    {
        loop
        {
            sum_weight += w[pack][0];
            // 積載量オーバー
            if sum_weight > p
            {
                sum_weight = 0;
                continue 'outer;
            }
    
            pack += 1;
            // 荷物を全て処理 -> pにおいて試行が可能である
            if pack == w.len() { return true; }
        }
    }

    return false;
}

fn main() {
    let n_k: Vec<u32> = read_vec();
    let w: Vec<Vec<u32>> = read_vec2(n_k[0]);
    let mut p_min = 0_u32;
    let mut p_max: u32 = 100000 * 10000;
    let mut p = p_max;
    let mut result = p_max;

    // 二分探索
    loop
    {
        //println!("Now p = {}, p_max = {}, p_min = {}, result = {}", p, p_max, p_min, result);

        if p == (p_max + p_min) / 2 { break; }

        // 平均を取る
        p = (p_min + p_max) / 2;

        // 割当可能であれば、下の山を探索する.
        if get_allocation(p, n_k[1], &w)
        {
            p_max = p;
            result = p;
            continue;
        }
        // 割当不可であれば、上の山を探索する.
        else
        {
            p_min = p;
            continue;
        }
    }

    println!("{}", result);
}
