pub fn main() {

    // セミコロンが文の区切り
    let a = 10;
    let b = 20;
    println!("{} {}", a, b);

    let a = { 10 + 20 };
    println!("{}", a);

    // 関数呼び出しも式
    let sum = add(10, 20);
    println!("{}", sum);

    // if文にはbool値を返す式が利用できる
    if a > 20 {
        println!("a is over 20");
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}