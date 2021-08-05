pub fn main() {
    // 文字列の扱い
    let x = 'x';
    println!("{}", x);
    let x = 'あ';
    println!("{}", x);
    let x = '😁';
    println!("{}", x);

    // 特殊文字
    let moji = '\''; // シングルクォート
    println!("{}", moji);
    let moji = '\\'; // バックスラッシュ
    println!("{}", moji);
    let moji = '\n'; // 改行
    println!("{}", moji);
    let moji = '\r'; // 復帰
    println!("{}", moji);
    let moji = '\t'; // タブ
    println!("{}", moji);

    // 文字列の一部を取り出す
    let s = "hello Rust world";
    let hello = &s[0..5]; // hello
    println!("{}", hello);
    let world = &s[11..]; // world
    println!("{}", world);
    let length = s.len();
    println!("{}", length); // 16

    // 文字列の結合
    let mut s = String::new(); // 空文字列を作る　変更可能のためmutをつける
    s.push_str("hello ");
    s.push_str("Rust ");
    s.push_str("world ");
    println!("{}", s);

    let s = format!("{} {} {}", "hello", "rust", "world2");
    println!("{}", s);

    // 日本語の扱い
    let ja = "あいうえお";
    println!("{}", ja); // 普通にできる

    // 実行時にエラー(1バイト目が区切り文字でないため)
    // let part = &ja[0..1];
    // println!("{}", part);

    let s = "こんにちは rust コードの世界";
    let hello = &s[0..15];
    let world = &s[21..];
    println!("{}", hello);
    println!("{}", world);
    println!("s length = {}", s.len()); // 17文字だが、39が表示

    // 日本語文字列の連結
    let mut s = String::new(); // 空文字列の追加
    s.push_str("こんにちは");
    s.push_str("rust ");
    s.push_str("コードの世界");
    println!("{}", s);

    // format で連結
    let s = format!("{} {} {}", "こんにちは", "rust", "コードの世界");
    println!("{}", s);

    // &strと$String型変換
    let s = "こんにちはRust コードの世界".to_string();
    println!("{}", s);
    let s = String::from(s);
    println!("{}", s);

    // 日本語文字列をスライスする
    let s = "こんにちはRust 🐈 日本語";

    // ベクターに変換する(可変の配列)
    let mut v: Vec<char> = Vec::new();
    for c in s.chars() {
        v.push(c);
    }
    println!("{:?}", v);

    // 8文字目から14文字目までを取得
    let v = &v[8..15];
    let mut s = String::new();
    for c in v {
        s.push(*c);
    }
    println!("{}", s);
}
