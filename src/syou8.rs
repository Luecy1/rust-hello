use std::num::ParseIntError;

// Resultの別名
type Result<T> = std::result::Result<T, ParseIntError>;

pub fn main() {
    // Personのインスタンス化
    let person = Person {
        id: 100,
        name: String::from("tanaka"),
        age: 50,
        addr: String::from("Tokyo"),
    };

    println!("Person.id {}", person.id);
    println!("Person.name {}", person.name);
    println!("Person.age {}", person.age);
    println!("Person.addr {}", person.addr);

    // 関数呼び出しで構造体を渡す
    print_person(&person);

    // Personのインスタンス化(変更可能)
    let mut person = Person {
        id: 100,
        name: String::from("tanaka"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    // 構造体を変更可能にする関数の呼び出し
    add_age(&mut person);
    println!("Person.id {}", person.id);
    println!("Person.name {}", person.name);
    println!("Person.age {}", person.age);
    println!("Person.addr {}", person.addr);

    // 戻り値が構造体の関数を呼び出す
    let person2 = new_person(1, "hoge");
    print_person(&person2);

    // ベクタで構造体を扱う
    let mut vec_person = vec![new_person(1, "vec1"), new_person(2, "vec2")];
    vec_person.push(new_person(3, "3ninme"));
    for person in &vec_person {
        println!("-------");
        print_person(person);
    }

    // フィールド名がない構造体
    struct Color(f32, f32, f32);
    let yellow = Color(1.0, 1.0, 0.0);
    println!("R {:.2}", yellow.0);
    println!("G {:.2}", yellow.1);
    println!("B {:.2}", yellow.2);

    // 構造体のサイズを調べる
    println!("Person size is {}", std::mem::size_of::<Person>());
    println!("Person size is {}", std::mem::size_of::<A>());

    // Personのメソッドを実行(引数なし)
    person.print();

    // Personのメソッドを実行(引数あり)
    person.print_t(true);
    person.print_t(false);

    println!("{}", person.to_str());

    // フィールドを変更する関数の呼び出し
    let mut person = Person {
        id: 100,
        name: String::from("tanaka"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    person.add_age();
    person.print();

    // new関数でPersonを作成
    let mut new_person_list = Vec::<Person>::new();
    new_person_list.push(Person::new("Tanaka", 20, "Tokyo"));
    new_person_list.push(Person::new("Satou", 21, "Osaka"));
    new_person_list.push(Person::new("Suzuki", 22, "Nagoya"));
    for person in new_person_list {
        person.print();
    }

    // Result型
    let result = "100".parse::<i32>();
    // Resultかた結果を取り出す
    match result {
        Ok(n) => {
            println!("100 = {}", n);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
    // 失敗するパターン
    let result = "100a".parse::<i32>();
    // Resultかた結果を取り出す
    match result {
        Ok(n) => {
            println!("100 = {}", n);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
    // 数値であることが保証できるとき
    let result = "100".parse::<i32>();
    println!("100 unwrap {}", result.unwrap());

    let half = half_number("xxx");
    match half {
        Ok(half) => println!("{}", half),
        Err(error) => println!("{}", error),
    }
}

// エラーなのにunwrapを使用した場合
// thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }', src/syou8.rs:117:22
fn half_number(s: &str) -> Result<i32> {
    match s.parse::<i32>() {
        Ok(i) => Ok(i / 2), // Okでwrapする
        Err(error) => Err(error)
    }
}


// 関数呼び出しで構造体を渡す
fn print_person(person: &Person) {
    println!("Person.id {}", person.id);
    println!("Person.name {}", person.name);
    println!("Person.age {}", person.age);
    println!("Person.addr {}", person.addr);
}

// 構造体の中身を変更できる関数
fn add_age(person: &mut Person) {
    person.age += 1;
}

// 構造体が戻り値の関数
fn new_person(id: i32, name: &str) -> Person {
    Person {
        id,
        name: name.to_string(),
        age: -1,
        addr: "Unknown".to_string(),
    }
}

struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

static mut PERSON_ID: i32 = 0;

// Personにメソッドを定義
impl Person {
    // 引数のないメソッド
    fn print(&self) {   // &selfは自分自身を参照するが変更はしない
        println!("Person.id {}", self.id);
        println!("Person.name {}", self.name);
        println!("Person.age {}", self.age);
        println!("Person.addr {}", self.addr);
    }

    // 引数のあるメソッド
    fn print_t(&self, private: bool) {
        if private {
            println!("id.name {}:{}", self.id, self.name);
        } else {
            println!("{}:{}:{}:{}", self.id, self.name, self.age, self.addr);
        }
    }

    // 戻り値のあるメソッド
    fn to_str(&self) -> String {
        format!("{}:{}:{}:{}", self.id, self.name, self.age, self.addr)
    }

    // フィールドを変更するメソッド
    fn add_age(&mut self) {
        self.age += 1;
    }

    // newメソッドを作る
    // RustはJavaみたいにnewがキーワードではない
    fn new(name: &str, age: i32, addr: &str) -> Person {
        let id = unsafe {   // グローバル変数を使うためunsafeで囲む
            PERSON_ID += 1;
            PERSON_ID
        };

        Person {
            id,
            name: name.to_string(),
            age,
            addr: addr.to_string(),
        }
    }
}

struct A {
    id: i32,
}