fn main() {
    // 変数の宣言、束縛(代入)
    let message1 = "Hello, world!";
    // !がついているものはマクロ、ついていないものは関数
    // こうするとコンパイラが関数を探すときに楽になる
    println!("{}", message1);

    // letは再代入できない
    // let message2 = "Hello, world! 2";
    // let mut message3 = "Hello, world! 3";
    // message3 = "Hello, world! 3-2";
}
