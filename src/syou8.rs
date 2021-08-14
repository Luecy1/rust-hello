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
