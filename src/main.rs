fn main() {
    // 所有権の移動
    let x = String::from("Hello");
    let y = x;

    println!("x :{}", x);
    println!("y :{}", y);
}