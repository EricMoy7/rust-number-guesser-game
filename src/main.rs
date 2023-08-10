use std::io;
use std::cmp::Ordering;
use rand::Rng;

use colored::Colorize;


fn main() {
    game();
    play_again_sequence();

}

fn play_again_sequence() {
    println!("Do you want to play again? (y/n)");
                let mut play_again: String = String::new();

                play_again.clear();
                io::stdin()
                    .read_line(&mut play_again)
                    .expect("Failed to read line");

                    
                    match play_again.trim() {
                        "n" | "N" => {
                            println!("Exiting game");
                        },
                        &_ => {
                            println!("Restarting game...");
                            main()
                        }
                    }
}

fn game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Uncomment to see secret number
    // println!("{}", &secret_number.to_string());

    loop {
        println!("Please input your number...");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break();
            },
        }
    }
}
