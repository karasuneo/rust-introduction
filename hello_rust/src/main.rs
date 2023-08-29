fn main() {
    // 変数の宣言、束縛(代入)
    // let message1 = "Hello, world!";
    // !がついているものはマクロ、ついていないものは関数
    // こうするとコンパイラが関数を探すときに楽になる
    // println!("{}", message1);

    // letは再代入できない
    // let message2 = "Hello, world! 2";
    // let mut message3 = "Hello, world! 3";
    // message3 = "Hello, world! 3-2";

    // 配列

    // ベクター型
    // let v = vec![1, 2, 3, 4, 5];
    // println!("v[0] is {}", v[100]);

    // エラー
    // march文は特定の値に対して異なるケースを指定し、値がどのケースにマッチするかに応じて異なる処理を行うための制御フロー構造
    // switch文と似ている
    let message = match might_fail() {
        Ok(_) => "処理に成功しました".to_string(),
        Err(cause_message) => cause_message
    };
    println!("{}", message);
}

fn always_error() -> Result<(), String> {
    Err("常にエラー".to_string())
}

// ()は引数として特になにも返さないという意味  
fn might_fail() -> Result<(), String> {
    let _result = always_error()?;
    Ok(())
}
