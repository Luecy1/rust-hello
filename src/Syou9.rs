pub fn main() {
    //　ジェネリクスの例
    let mut v: Vec<&str> = Vec::new();
    v.push("aiueo");
    println!("{:?}", v);

    let v = [1, 2, 3, 4, 5];
    print(&v);
}

// ジェネリクスを使った関数
fn print<T>(a: &[T]) where T: std::fmt::Debug { // whereでトレイトを指定する
    print("a is");
    for x in a {
        print!("{:?}", x);
    }
    println!();
}