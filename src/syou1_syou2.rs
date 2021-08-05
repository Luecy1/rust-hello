fn main() {

    // 変数宣言
    let name = "Tanaka taro";
    let age = 20;
    println!("Hello, world!");

    println!("name:{} age:{}", name, age);

    let string1 = String::from("Hello");
    let string2 = String::from("Hello");
    let string3 = String::from("Hello");

    let formatted = format!("{} {} {}", string1, string2, string3);
    println!("{}", formatted);

    // 関数の呼び出し
    println!("{}", add(1, 3));

    // タプル
    let tuple = ("a", 12);
    println!("{}", tuple.0);

    // 配列
    let season = ["春", "夏", "秋", "冬"];
    println!("最初の季節 {}", season.first().unwrap());
    println!("最後の季節 {}", season.last().unwrap());

    // 配列外アクセス
    // println!("out of bound {}", season[10])
}


// 関数の宣言
fn add(x: i32, y: i32) -> i32 {
    x + y
}