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
    for track in 0..k
    {
        
    }

    return false;
}

fn main() {
    let n_k: Vec<u32> = read_vec();
    let w: Vec<Vec<u32>> = read_vec2(n_k[0]);
    let mut p_min = 0_u32;
    let mut p_max = 100000_u32;

    // 二分探索
    loop
    {
        if p == p_min { break; }

        // 平均を取る
        let p = (p_min + p_max) / 2;

        // 割当可能であれば、下の山を探索する.
        if get_allocation(p, n_k[1], &w)
        {
            p_max /= 2;
            continue;
        }
        // 割当不可であれば、上の山を探索する.
        else
        {
            p_min = p;
            continue;
        }
    }
}
