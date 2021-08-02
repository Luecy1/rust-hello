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
    println!("{}", x)    // 借用
}

fn string_length(s: &String) -> usize {
    s.len()
}