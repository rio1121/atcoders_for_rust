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

/*
    配列を降順にソートして先頭から破壊的に要素を取り出していく手法
*/
fn main() {
    let number_of_cards: i32 = read();
    let mut cards: Vec<i32>  = read_vec();

    let mut alice_points: i32 = 0;
    let mut   bob_points: i32 = 0;

    // sort on desc
    cards.sort();
    cards.reverse();

    loop
    {
        // alice turn
        alice_points += cards[0];
        cards.remove(0);
        if cards.len() <= 0 { break; }

        // bob turn
        bob_points += cards[0];
        cards.remove(0);
        if cards.len() <= 0 { break; }
    }

    println!("{}", alice_points - bob_points);
}
