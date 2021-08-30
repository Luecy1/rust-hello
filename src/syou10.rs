pub fn main() {
    let a = Person { name: "masuda", age: 50 };
    // 借用させる
    print_a(&a);
    // 変数aはmainに残る
    println!("main: a is {:?}", a);

    let a = Person { name: "masuda", age: 50 };
    // 移動させる
    move_a(a);
    // 変数aはmainで使用できない(コンパイルエラー)
    // println!("main: a is {:?}", a);

    // 変数aを変数xに借用させる
    let a = Person { name: "masuda", age: 50 };
    let x = &a;
    // aとxの両方使える
    println!("main: a is {:?}", a);
    println!("main: x is {:?}", x);

    // 変数aを変数xに移動させる
    let a = Person { name: "masuda", age: 50 };
    let x = a;
    // 変数aは使えなくなる(コンパイルエラー)
    // println!("main: a is {:?}", a);
    println!("main: x is {:?}", x);

    // 構造体の内容を変更する関数の呼び出し
    let mut a = Person { name: "masuda", age: 50 };
    println!("a is {:?} : before add_age", a);
    add_age(&mut a);
    println!("a is {:?} : after add_age", a);
}

// 借用する
fn print_a(a: &Person) {
    println!("print_a: a is {:?}", a);
}

// 移動する
fn move_a(a: Person) {
    println!("print_a: a is {:?}", a);
}

// 構造体の内容を変更する
fn add_age(a: &mut Person) {
    a.age += 1;
}

// person構造体
#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32,
}