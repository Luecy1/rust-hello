pub fn main() {
    let a = 10;
    let b = 20;
    if a < b {
        println!("a > b is true");
    } else {
        println!("a > b is ng")
    }

    // ifは値を返す
    let x = if a < b {
        "a > b"
    } else {
        "a < b"
    };
    println!("{}", x);
}