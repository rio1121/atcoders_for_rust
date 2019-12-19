// 解説動画視聴後に解答

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
    let a_b_x: Vec<f64> = read_vec();
    let side_length  = a_b_x[0];
    let box_height   = a_b_x[1];
    let water_volume = a_b_x[2];
    let result_degree: f64;
    let theta: f64;

    // 垂直状態で実際に入っている水を容器の横から見たときの面積
    let water_area: f64 = water_volume / side_length;
    // 横から見たときの容器の面積
    let box_area: f64 = side_length * box_height;
    // こぼれる寸前まで傾けた時に、容器の横から見たときの水が入っていない部分の面積
    let blank_area: f64 = box_area - water_area;
    
    // 場合分け
    if water_area >= box_area / 2.0
    {
        // こぼれる寸前まで傾けた時における、直角三角形の辺の高さ
        let height: f64 = (blank_area * 2.0) / side_length;

        // こぼれる寸前の角度
        theta = (height / side_length).atan();
    }
    else
    {
        // こぼれる寸前まで傾けた時における、直角三角形の辺の幅
        let width: f64 = (water_area * 2.0) / box_height;

        // こぼれる寸前の角度
        theta = (box_height / width).atan();
    }

    // ラジアン => 度数法
    result_degree = (theta / (std::f64::consts::PI * 2.0)) * 360.0;
    println!("{}", result_degree);
}
