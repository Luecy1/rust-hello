pub fn main() {
    //　ジェネリクスの例
    let mut v: Vec<&str> = Vec::new();
    v.push("aiueo");
    println!("{:?}", v);
}

// ジェネリクスを使った関数