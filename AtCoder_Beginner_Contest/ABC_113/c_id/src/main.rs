use std::cmp::Ordering;

/// Equivalent to std::lowerbound and std::upperbound in c++
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                Ordering::Equal | Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }

    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                }
                Ordering::Greater => {
                    high = mid;
                }
            }
        }
        low
    }
}

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
    let nm: Vec<usize> = read_vec();
    let n = nm[0];
    let m = nm[1];

    let mut data: Vec<Vec<u64>> = vec![Vec::new(); m];
    for i in 0..m
    {
        let e: Vec<u64> = read_vec();
        data[i] = e;
    }

    // Piの値ごとの配列に分ける...元のデータとは別に用意しなければならない(最後のprintで使うため)
    let mut vals = vec![Vec::new(); n];
    for e in &data
    {
        vals[(e[0] - 1) as usize].push(e[1]);
    }

    // Piごとにソート
    for i in 0..n
    {
        vals[i].sort();
    }

    // 二分探索を用いて求める結果を出力
    for i in 0..m
    {
        print!("{:06}", data[i][0]);
        println!("{:06}", vals[data[i][0] as usize - 1].lower_bound(&data[i][1]) + 1);
    }
}
