fn main() {
    // 所有権の移動
    // let x = String::from("Hello");
    // let y = x;
    //
    // println!("x :{}", x);
    // println!("y :{}", y);

    // 借用の例
    let x = String::from("Hello");
    // println!("Hello no length {}", string_length(x));
    // println!("{}", x)    // Error
    println!("Hello no length {}", string_length(&x));
    println!("{}", x);    // 借用

    // 変数（）再定義
    let mut x = 10; // mutをつけると変更ができる
    x += 10;
    println!("{}", x);

    // シャドーイング
    let x = 10;
    println!("{}", x);
    let x = 10; // 同じ変数名を再定義できる
    println!("{}", x);

    // スコープ
    let x = 100;
    println!("{}", x);  // 100
    {
        let x = 200;
        println!("{}", x);  // 200
    }
    println!("{}", x);  // 100
}

fn string_length(s: &String) -> usize {
    s.len()
}