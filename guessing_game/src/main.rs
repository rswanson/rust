use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess A Number Game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("DEBUG>>Secret Number: {secret_number}");

    loop {
        println!("Input your guess: ");
        let mut guess = String::new(); 
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please only enter numbers!");
                continue;
            }
        };
            
        println!("You guessed {}",guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }    
    }
}
