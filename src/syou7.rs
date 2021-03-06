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
    let array: [u8; 16] = [0; 16];
    print!("0 init ->");
    for x in array {
        print!("{} ", x);
    }
    println!();
    let vector = vec![1, 2, 3, 4];
    println!("{}", vector[0]);
    println!("{:?}", vector.get(1));    // {:?}はデバッグ表記
    println!("{}", vector.get(2).unwrap());

    // Vectorの要素の増減
    let mut vector: Vec<i32> = Vec::new();
    println!("vector {:?}", vector);
    vector.push(10);
    vector.push(20);
    vector.push(30);
    vector.push(40);
    println!("vector {:?}", vector);
    // 要素の削除
    println!("{:?}", vector.pop());
    println!("{:?}", vector.pop());
    println!("{:?}", vector.pop());
    println!("vector {:?}", vector);

    // forの利用
    let v = vec![1, 2, 3, 4, 5];
    for x in v {
        println!("{}", x);
    }
    // イテレータの利用
    let v = vec![1, 2, 3, 4, 5];
    for x in v.iter() {
        println!("{}", x);
    }

    let mut p = v.iter();
    println!("{:?}", p);
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next());
    println!("p is {:?}", p.next()); // 6回目はNone

    // while + iter
    let mut p = v.iter();
    while let Some(x) = p.next() {
        println!("{}", x);
    }

    // mapを使う
    let v = vec![1, 2, 3, 4, 5];
    let lst = v.iter().map(|x| x * 10);
    for i in lst {
        println!("i is {}", i);
    }

    v.iter().map(|x| x * 10).for_each(|x| {
        println!("{:?}", x);
    })
}