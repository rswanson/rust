fn main() {

    let x = five();

    print_number(x,'x'); 

    printt_labeled_measurements(x, 'h');

    let y = {
        let x = 3;
        x + 1
    };

    print_number(y,'y')
}

fn five() -> i32 {
    5
}
fn print_number(x: i32, var_name: char) {
    println!("The value of {var_name} is: {x}");
}

fn printt_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}