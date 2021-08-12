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
}

// 関数呼び出しで構造体を渡す
fn print_person(person: &Person) {
    println!("Person.id {}", person.id);
    println!("Person.name {}", person.name);
    println!("Person.age {}", person.age);
    println!("Person.addr {}", person.addr);
}

struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}
