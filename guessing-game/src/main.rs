use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() -> () {
    println!("\nGuess the number between 1 and 100!\n");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    let mut num_attempts = 0;

    loop {
        num_attempts += 1;

        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("\nToo small.\n"),
            Ordering::Greater => println!("\nToo big.\n"),
            Ordering::Equal => {
                println!("\nYou win! It took you {num_attempts} attempts.\n");
                break;
            }
        }
    }
}
