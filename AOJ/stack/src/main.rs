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
    let formula: Vec<String> = read_vec();
    let mut stack: Vec<i32> = Vec::new();

    // ポーランド記法で表された数式を計算
    for value in formula
    {
        if value == "+"
        {
            let x = stack.pop().unwrap();
            let y = stack.pop().unwrap();
            stack.push(x + y);

            continue;
        }

        if value == "-"
        {
            let x = stack.pop().unwrap();
            let y = stack.pop().unwrap();
            stack.push(y - x);

            continue;
        }

        if value == "*"
        {
            let x = stack.pop().unwrap();
            let y = stack.pop().unwrap();
            stack.push(x * y);

            continue;
        }

        stack.push(value.parse::<i32>().unwrap());
    }

    println!("{}", stack[0]);
}
