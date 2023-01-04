fn main() {
    let number_vec = vec![20,30,40,100,50];
    let char_vec = vec!['a','b','c','z','y'];
    let biggest_number = largest(&number_vec);
    let biggest_char = largest(&char_vec);

    println!("The biggest number is: {biggest_number}");
    println!("The biggest char is: {biggest_char}");
}

fn largest_i32(list : &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest

}

// generic with a trait - neat. 
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}