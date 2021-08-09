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
}