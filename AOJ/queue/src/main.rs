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

use std::collections::VecDeque;

fn main() {
    let n_q: Vec<u32> = read_vec();
    let proc_data: Vec<Vec<String>> = read_vec2(n_q[0]);
    let mut queue = VecDeque::new();
    let mut elapsed_time = 0;

    // 両端キュー構造へproc_data[i]の値を格納する インデックス付きループで[i + 1番目のプロセス, 残り処理時間]のペアで格納.
    for (index, element) in proc_data.iter().enumerate()
    {
        let tmp_vec = vec![index as u32, element[1].parse::<u32>().unwrap()];
        queue.push_back(tmp_vec);
    }

    // ラウンドロビンスケジュール操作
    loop
    {
        // キューが空であれば終了
        if queue.len() <= 0 { break; }

        // pop操作(先頭のデータを取り出す)
        let mut poped_vec = queue.pop_front().unwrap();

        // 残り処理時間からn_q[1]分引く
        if poped_vec[1] > n_q[1] {
            poped_vec[1] -= n_q[1];
            elapsed_time += n_q[1];
        }
        // n_q[1]以下の値であれば、残り時間を消費してそのプロセスに対する処理を終了.
        else{
            elapsed_time += poped_vec[1];
            println!("{} {}", proc_data[poped_vec[0] as usize][0], elapsed_time);
            continue;
        }

        // 更新後のpoped_vecを末尾に追加
        queue.push_back(poped_vec);
    }
}

