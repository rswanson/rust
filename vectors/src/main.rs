fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("the third item in the vector is: {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("the third item in the vector is {}", third),
        None => println!("There is no third element"),
    }

    let v = vec![1, 2, 3];

    for i in &v {
        println!("{i}");
    }

    let mut v = vec![1, 2, 3];

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    // vectors only hold same type'd values, but if you use an enum with a vector you can do multitype
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Cell data")),
    ];
}
