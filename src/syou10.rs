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

    // 所有権を失った変数を参照するこどでエラーとなるコード
    let a = Person { name: "masuda", age: 50 };
    let mut x = a;
    println!("x is {:?} before add_age", x);
    add_age(&mut x);
    println!("x is {:?} after add_age", x);
    // add_age(&mut a); 所有権を失っているので参照するとエラーになる
    // println!("a is {:?} after add_age", a);

    // 不変な変数aを可変な変数として参照することでエラーとなるコード
    let a = Person { name: "masuda", age: 50 };
    let mut x = &a;
    println!("x is {:?}", x);
    // x.age += 1; error[E0594]: cannot assign to `x.age` which is behind a `&` reference
    // println!("x is {:?}", x);

    // 可変の変数から可変の変数を参照することで正しくなるコード
    let mut a = Person { name: "masuda", age: 50 };
    let mut x = &mut a;
    println!("x is {:?}", x);
    x.age += 1;
    println!("x is {:?}", x);
    add_age(&mut x);
    println!("x is {:?}", x);

    // 数値の例
    let a = 100;
    println!("a is {}", a);
    let x = a;  // 値のコピー
    println!("x is {}", x);
    let y = a;  // 値のコピー
    println!("y is {}", y);
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