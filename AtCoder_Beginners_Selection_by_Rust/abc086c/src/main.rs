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
fn read_vec2<T: std::str::FromStr>(n: i32) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn main() {
    let n: i32 = read();
    let t_x_y: Vec<Vec<i32>> = read_vec2(n);
    // 現在地の得点
    let mut now = [0, 0];
    // 現在の時刻
    let mut elapsed_time: i32 = 0; 
    let mut result = "Yes";

    for i in 0..n as usize
    {
        // 必要な得点を求める: 行先の得点 - 現在値の得点の絶対値
        let needed_point = (t_x_y[i][1] - now[0]).abs() + (t_x_y[i][2] - now[1]).abs();
        // 必要な得点が経過秒数を超えているか、偶奇が一致していない場合は移動不可能
        if (needed_point > (t_x_y[i][0] - elapsed_time)) || (needed_point % 2 != (t_x_y[i][0] - elapsed_time) % 2)
        {
            result = "No";
            break;
        }
        // 値の更新
        now = [t_x_y[i][1], t_x_y[i][2]];
        elapsed_time = t_x_y[i][0];
    }

    println!("{}", result);
}
