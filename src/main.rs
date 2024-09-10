use std::cmp::Ordering;
use rand::Rng;
fn main() {
    let random_number: u32 = rand::thread_rng().gen_range(1..100);
    println!("Guess the number!");
    //println!("{random_number}");
    loop {
        println!("Please input your guess.");
        let mut guess: String = "".to_string();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {guess}");
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
