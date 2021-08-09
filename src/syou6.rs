pub fn main() {
    let a = 10;
    let b = 20;
    if a < b {
        println!("a > b is true");
    } else {
        println!("a > b is ng")
    }

    // ifは値を返す
    let x = if a < b {
        "a > b"
    } else {
        "a < b"
    };
    println!("{}", x);

    // 繰り返し
    let vec = vec![10, 20, 30, 40, 50];
    for v in &vec {
        print!("{} ", v);
        let vp: i32 = *v;   // *をつけて参照外し
    }
    println!();

    // イテレータを使う
    for i in vec.iter() {
        println!("{}", i);
    }

    // インデックス付きでループ タプルで受け取る
    for (i, x) in vec.iter().enumerate() {
        println!("index:{} content{}", i, x);
    }

    // 指定数ループ
    for i in 0..10 {
        println!("{}", i);
    }

    // 途中で先頭に戻る
    for x in 0..10 {
        if x % 2 == 0 {
            continue;
        }
        println!("{}", x);
    }

    let mut x = 0;
    while x < 10 {
        println!("{}", x);
        x += 1;
    }

    let mut x = 0;
    // 無限ループ構文
    loop {
        if x > 10 {
            break;
        }
        x += 1;
    }

    // 型列挙の利用
    let lang = LANG::JAPANESE;
    println!("LANG::JAPANESE {:?}", lang);

    // matchを使って分岐
    let m = match lang {
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH => "英語",
        LANG::CHINESE => "中国語",
        LANG::FRANCH => "フランス語",
    };
    println!("{}", m);

    // _を使用
    let m = match lang {
        LANG::JAPANESE => "日本語",
        _ => "日本語以外",
    };
    println!("{}", m);

    // Optional
    let x = Some(10);
    let v = match x {
        Some(i) => i,
        None => 0
    };
    println!("{}", v);
}

// 列挙
#[derive(Debug)]    // デバッグ表示を有効にする
enum LANG {
    JAPANESE,
    ENGLISH,
    CHINESE,
    FRANCH,
}