
use std::io;
use crate::grid;
pub fn run(){
    loop {
        println!("Do you want to play again?  (y/n)");
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).expect("Failed to read line");
            let answer = answer.trim();

            if answer == "y" || answer == "yes"{
                grid::run();
                break;
            } else if answer == "n"|| answer == "no" || answer == "N" || answer == "NO" {
                println!("Thanks for playing. Bye");
                break;
            } else {
                print!("Invalid input.");
                
            }
    }
}