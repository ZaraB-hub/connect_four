use std::io;
use crate::custom;
use crate::logic;
use crate::playagain;
use crate::savable::GameState;
use crate::continue_game::continue_game;

use std::fs::File;
use std::io::prelude::*;


pub fn run(){

loop{
        println!("Please choose your playing board:");
        println!("1. Default board (6x7)");
        println!("2. Make your own custome board ");
        println!("L. Load a previous game");
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
            "L" | "l" =>{
                println!("load game");
                let mut file = File::open("state.txt").unwrap();  // open the file
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();  // read the contents into a string
                let state: GameState = serde_json::from_str(&contents).unwrap();  // deserialize the string into a GameState object
                continue_game(state);
                playagain::run();
                break;
            }
            "Q" | "q" => {println!("You chose to quit the current game"); break;}
            _ => println!("Invalid option"),
        }
}

}