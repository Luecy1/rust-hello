pub fn main() {
    //　ジェネリクスの例
    let mut v: Vec<&str> = Vec::new();
    v.push("aiueo");
    println!("{:?}", v);

    let v = [1, 2, 3, 4, 5];
    print(&v);

    let rect = Rectangle {
        width: 10.0,
        height: 20.0,
    };
    let tri = Triangle {
        width: 10.0,
        height: 20.0,
    };
    let cir = Circle {
        radius: 10.0,
    };

    println!("{}", rect.calc_area());
    println!("{}", tri.calc_area());
    println!("{}", cir.calc_area());
}

// ジェネリクスを使った関数
fn print<T>(a: &[T]) where T: std::fmt::Debug { // whereでトレイトを指定する
    print!("a is");
    for x in a {
        print!("{:?}", x);
    }
    println!();
}

// 四角形
struct Rectangle {
    width: f32,
    height: f32,
}

// 三角形
struct Triangle {
    width: f32,
    height: f32,
}

// 円
struct Circle {
    radius: f32,
}

trait CalcArea {
    fn calc_area(&self) -> f32;
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f32 {
        self.width * self.height
    }
}

impl CalcArea for Triangle {
    fn calc_area(&self) -> f32 {
        self.width * self.height * 0.5
    }
}

impl CalcArea for Circle {
    fn calc_area(&self) -> f32 {
        self.radius * self.radius * 3.14
    }
}