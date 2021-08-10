pub fn main() {
    let array = [1, 2, 3, 4, 5];
    println!("{}", array[0]);
    println!("{}", array[4]);
    println!("{}", array.len());

    // 文字列の配列
    let array = ["one", "two", "three", "four", "five"];
    println!("{}", array[0]);
    println!("{}", array[4]);
    println!("{}", array.len());

    // 型を指定するときは要素数も指定する
    let array: [u8; 5] = [0x10, 0x20, 0x30, 0x40, 0x50];
    println!("{}", array[0]);
    println!("{}", array[4]);
    println!("{}", array.len());

    // 0で初期化する
    let mut array: [u8; 16] = [0; 16];
    print!("0 init ->");
    for x in array {
        print!("{} ", x);
    }
    println!()
}