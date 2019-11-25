fn run() {
    // initiallize vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    // update
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // access element of vector
    let mut v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    // 範囲外アクセスするとNoneを返す．
    let third: Option<&i32> = v.get(2);
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    // Enumを使って複数の型を保持する．
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn main() {
    run();
}
