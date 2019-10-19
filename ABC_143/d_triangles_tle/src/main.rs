// 未回答: TLE
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
    let number: i32 = read();
    let mut sticks: Vec<i32> = read_vec();
    
    let mut count_of_triangles: i32 = 0;
    
    loop
    {
        for b in 1..sticks.len() {
            for c in (b + 1)..sticks.len() {
                count_of_triangles += 
                if (sticks[0] < sticks[b] + sticks[c]) &&
                    (sticks[b] < sticks[c] + sticks[0]) &&
                    (sticks[c] < sticks[0] + sticks[b]) { 1 } else { 0 };
            }
        }
        sticks.remove(0);
        if sticks.len() < 3 { break; }
    }
    
    println!("{}", count_of_triangles);
}
