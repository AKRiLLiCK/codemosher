use std::io;

fn main() {
    println!("Welcome to codemosher, do you want to play a game? [y/n]");
    let mut start_prompt = String::new();

    io::stdin()
        .read_line(&mut start_prompt)
        .expect("Failed to get prompt");

    match start_prompt.trim() {
        "y" => generate_game(),
        "n" => println!("Alright, bye bye!"),
        _ => println!("Prompt not understood: {}", start_prompt.trim()),
    }
}

fn generate_game() {
    let code: u32 = rand::random_range(0..=10);
    println!("Guess the number between 0 and 10!");

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get prompt");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This isn't a number! Please try again.");
                continue;
            }
        };

        if check_code(guess, code) {
            break;
        }
    }
}

fn check_code(guess: u32, code: u32) -> bool {
    println!("Your guess: {}", guess);

    match guess.cmp(&code) {
        std::cmp::Ordering::Less => {
            println!("Bigger!");
            false
        },
        std::cmp::Ordering::Equal => {
            println!("You got it!");
            true
        },
        std::cmp::Ordering::Greater => {
            println!("Smaller!");
            false
        },
    }
}