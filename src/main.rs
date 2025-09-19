use colored::Colorize;
use rand::Rng;
use std::io;

fn main() {
    loop {
        println!("\n{}", "🎮 Welcome to Guess Game 🎮".bold());
        println!("1. Play the game");
        println!("2. About");
        println!("3. Quit");

        // Get menu choice
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Error reading input");
        let choice = choice.trim();

        match choice {
            "1" => play_game(),
            "2" => show_about(),
            "3" => {
                println!("{}", "👋 Bye homiee! See you later.".cyan());
                break;
            }
            _ => println!("{}", "Invalid choice bruh, try again 😭".yellow()),
        }
    }
}

fn play_game() {
    println!("{}", "🔥 Starting the game... 🔥".bold());
    let secret = rand::rng().random_range(1..1001);

    loop {
        println!("Enter your guess (1-1000):");

        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Error reading input");

        let num: u32 = match num.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("{}", "Bruh... that ain’t even a number 😭".yellow());
                continue;
            }
        };

        if num > secret {
            println!("{}", "Too thicc, dial it down 📉".red());
        } else if num < secret {
            println!("{}", "Too smol, pump it up 📈".red());
        } else {
            println!("{}", "W no cap! You nailed it 🏆✨".green());
            break;
        }
    }
}

fn show_about() {
    println!("\n{}", "📜 About NumbaWumbo 📜".bold());
    println!(
        "{}",
        "A goofy lil' terminal game where you guess a number between 1 and 1000.".bright_blue()
    );
    println!(
        "{}",
        "Built in Rust 🦀 with RNG magic, for all the homies out there.".bright_blue()
    );
    println!(
        "{}",
        "Follow me on X (Twitter): @sudosuanjal".bright_purple()
    );
}
