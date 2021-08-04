pub fn main() {

    // 文字列の扱い
    let x = 'x';
    println!("{}", x);
    let x = 'あ';
    println!("{}", x);
    let x = '😁';
    println!("{}", x);

    // 特殊文字
    let moji = '\'';    // シングルクォート
    println!("{}", moji);
    let moji = '\\';    // バックスラッシュ
    println!("{}", moji);
    let moji = '\n';    // 改行
    println!("{}", moji);
    let moji = '\r';    // 復帰
    println!("{}", moji);
    let moji = '\t';    // タブ
    println!("{}", moji);
}