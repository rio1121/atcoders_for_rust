// 標準入力から一行を読み取り、指定の型に変換する
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
    
fn main() {
    let s: String = read();
    let mut result = "NO";
    // 逆順にしてVec化する
    let mut s_vec: Vec<char> = s.chars().rev().collect();

    loop
    {
        // 文字数が丁度0になれば成功
        if s_vec.len() == 0
        {
            result = "YES";
            break;
        }

        // 4文字以下は強制終了
        if s_vec.len() <= 4 { break; }

        // dream
        if s_vec[0] == 'm'
        {
            // 5文字削除
            let drain = s_vec.drain(..5);
            if drain.collect::<String>() == "maerd" { continue; } else { break; }
        }

        // erase
        if s_vec[0] == 'e'
        {
            // 5文字削除
            let drain = s_vec.drain(..5);
            if drain.collect::<String>() == "esare" { continue; } else { break; }
        }

        // dreamer,eraser
        if s_vec[0] == 'r'
        {
            if s_vec[2] == 'm'
            {
                // 2文字削除
                let drain = s_vec.drain(..2);
                if drain.collect::<String>() == "re" { continue; } else { break; }
            }
            // 1文字削除
            let drain = s_vec.drain(..1);
            continue;
        }

        // いずれの文字でもなければ強制終了
        break;
    }

    println!("{}", result);
}