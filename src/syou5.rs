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

    // 割り算と整数、実数の違い
    let ans = 10 / 3; // ans = 3
    println!("{}", ans);

    let ans = 10.0 / 3.0; // ans = 3.3333333333333335
    println!("{}", ans);

    // ビット演算
    let a: u8 = 0b1111;
    let b: u8 = 0b0011;
    println!("a & b = {:04b}", a & b);
    println!("a | b = {:04b}", a | b);

    // シフト演算子
    let a: u8 = 0x02;
    println!("a << 1 = {}", a << 1);
    println!("a >> 1 = {}", a >> 1);

    // 論理演算子
    let a = true;
    let b = false;
    println!(" a && b = {}", a && b);
    println!(" a || b = {}", a || b);

    no_param();
    one_param(1);
    two_param(2, 3);
    println!("{}", return_value());
    str_param("hoge");

    let v = vec![0, 1, 2, 3, 4, 5];
    let sum = vec_param(&v);
    println!("sum = {}", sum);

    let renban = return_vec(5);
    println!("{:?}", renban);

    let mut v = vec![0, 1, 2, 3, 4, 5];
    vec_change(&mut v);
    println!("{:?}", v);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 引数も返り値もない関数
fn no_param() {
    println!("no_param called");
}

// 引数が一つの関数
fn one_param(x: i32) {
    println!("{}", x);
}

// 引数が2つの関数
fn two_param(x: i32, y: i32) {
    println!("x y {} {}", x, y);
}

// 戻り値がある関数
fn return_value() -> i32 {
    10
}

// 文字列を受け取る関数
fn str_param(s: &str) {
    println!("called {}", s);
}

// ベクターを受け取る関数
fn vec_param(v: &Vec<i32>) -> i32 {
    println!("vec_param");
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}

// ベクターを返す関数
fn return_vec(max: i32) -> Vec<i32> {
    println!("called vec_param");
    let mut vec = Vec::new();
    for i in 0..max {
        vec.push(i);
    }
    vec
}

// ベクターの中身を変更する関数
fn vec_change(v: &mut Vec<i32>) {
    println!("called vec_change");
    for i in v {
        *i = *i * 10;
    }
}