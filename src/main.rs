mod generation;
mod player;

use std::io::{self, Write};
use std::{thread, time};

fn main() {
    let map: generation::map::Map =
        generation::map::Map::new(generation::location::Location::Plains);
    println!("Printing the {} map.", map.name);
    map.print_layout();
    let mut new_player = player::Player::new("Sage".to_string(), 1);
}

// fn main() {
//     let mut new_player = Player::new("Sage".to_string(), 1);
//     let intro = "Welcome to the beautiful world of Command Line! You are the first adventurer that has entered this world in over a decade!";
//     narrator(intro);
//     let intro = "We expect everything from you and will provide you with nothing but the command line. Unsure what to do? That's unfortunate!";
//     narrator(intro);
//     loop {
//         print!("> ");
//         io::stdout().flush().unwrap();

//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();

//         let trimmed_input = input.trim();

//         if trimmed_input == "exit" {
//             println!("Exiting the game. Goodbye!");
//             break;
//         }

//         println!("You said: {}", trimmed_input);
//     }
// }

// fn narrator(s: &str) {
//     for (i, c) in s.chars().enumerate() {
//         print!("\x1b[32m{c}\x1b[0m");
//         std::io::stdout().flush().expect("Flushing to succeed");
//         std::thread::sleep(std::time::Duration::from_millis(10));

//         if i == s.len() - 1 {
//             print!("\n");
//             std::io::stdout();
//         }
//     }
// }
