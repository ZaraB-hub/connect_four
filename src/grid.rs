use std::io;
use crate::custom;
use crate::logic;
use crate::playagain;


pub fn run(){

loop{
        println!("Please choose your playing board:");
        println!("1. Default board (6x7)");
        println!("2. Make your own custome board ");
        println!("Q. Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        match input {
            "1" =>{logic::run(6,7);
                playagain::run();
                    break;}
            "2" =>{custom::run();
                playagain::run();
                break;}
            "Q" | "q" => {println!("You chose to quit the current game"); break;}
            _ => println!("Invalid option"),
        }
}

}