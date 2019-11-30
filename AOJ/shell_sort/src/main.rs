// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// 配列要素をスペースで区切って出力する。
fn print_vec(v: Vec<u32>)
{
    let mut is_first: bool = true;

    for element in v.iter()
    {
        if !is_first { print!(" "); }
        print!("{}", element);
        is_first = false;
    }

    println!("");
}

// 距離付き挿入ソートを行う.
fn insertion_sort(number: u32, distance: usize, array: &mut Vec<u32>) -> u32
{
    let mut count: u32 = 0;

    for i in distance..number as usize
    {
        let value = array[i];
        let mut j = (i - distance) as i32;
        while j >= 0 && array[j as usize] > value
        {
            array[j as usize + distance] = array[j as usize];
            j = j - distance as i32;
            count += 1;
        }
        array[(j + distance as i32) as usize] = value;
    }

    return count;
}

// シェルソートを行う
fn shell_sort(number: u32, array: &mut Vec<u32>) -> u32
{
    let mut count: u32 = 0;
    // 挿入ソートの必要回数
    let mut attempt: u32 = 1;
    // 挿入ソートの距離を格納した配列
    let mut distance_array: Vec<u32> = vec![1];
    // 必要回数を求める。
    loop
    {
        // 3^k - 1 / 2が配列の要素数n以上を取る最小のkを求める。
        if (3_u32.pow(attempt + 1) - 1) / 2 > number { break; }
        attempt += 1;
        // 3^k - 1 / 2の値を0番目(先頭)に挿入する.
        distance_array.insert(0, (3_u32.pow(attempt) - 1) / 2);
    }

    println!("{}", attempt);
    print_vec(distance_array.to_vec());

    for i in 0..attempt as usize
    {
        count += insertion_sort(number, distance_array[i] as usize, array);
    }

    return count;
}

fn main() {
    let n: u32 = read();
    let mut a: Vec<u32> = Vec::new();
    for i in 0..n as usize
    {
        a.push(read());
    }

    println!("{}", shell_sort(n, &mut a));
    for i in 0..n as usize
    {
        println!("{}", a[i]);
    }
}
