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

fn main() {
    let n: u32 = read();
    // []1: i番目の証言者
    // []2: 対象の証言者i(1から始まることに注意. つまりi + 1)
    // []3: どんな人であるか 0:不親切 1:正直者
    let mut a: Vec<Vec<Vec<u32>>> = Vec::new();
    a.resize(n as usize, Vec::new());

    for index in 0..n
    {
        let a_n: u32 = read();
        for jndex in 0..a_n as usize
        {
            let x_y: Vec<u32> = read_vec();
            a[index as usize].push(x_y);
        }
    }

    

    println!("{:?}", a);
}
