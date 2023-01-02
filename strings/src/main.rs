fn main() {
    // definitions
    let mut s = String::new();
    let data = "initial data";
    let s = data.to_string();
    println!("the data: {s}");
    // comparison
    let s1 = "initial context".to_string();
    let s2 = String::from("initial context");
    if s1 == s2 {
        println!("these are equal");
    } else {
        println!("not equal");
    }

    // concatenation 
    let mut s = String::from("foo");
    s.push_str(" bar");
    println!("{s}");

    let mut s = String::from("foo");
    let s2 = " bar";
    s.push_str(s2);
    println!("{s}");
    println!("{s2}");

    let mut s = String::from("foo");
    let s2 = 'j';
    s.push(s2);
    println!("{s}");
    println!("{s2}");

    let s1 = String::from("Hello, "); 
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 can no longer be used due to being moved into s1, s2 still valid 
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // takes no ownership 

    println!("{s}"); 

    // iterating 

    // iterates over individual unicode scalar values
    for c in "Зд".chars() {
        println!("{c}");
    }
    // iterates over each byte needed to store the string 
    for b in "Зд".bytes() {
        println!("{b}");
    }

}
