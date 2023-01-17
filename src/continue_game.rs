use crate::savable::GameState;
use crate::win::check_win;
use std::fs::File;
use std::io::prelude::*;
use std::io;


pub fn continue_game(state: GameState) {
    let mut grid = state.grid;
    let mut player1 = state.player1;
    let mut player2 = state.player2;
    let mut current_player = state.current_player;
    let max_turns = state.max_turns;
    let mut moves = state.moves;

    loop {

        let num_cols = grid.first().unwrap().len();

        for i in 0.. num_cols {
            print!("   {}   ", i);
        }
        println!(" ");

        for row in grid.iter() {
            print!("│");
            for element in row.iter() {
                print!("  {}   ", element);
                print!("│");
            }
            println!(" ");
        }
        
        println!("Enter a column number (0-{}):", grid[0].len() - 1);

        let mut column_str = String::new();
        io::stdin()
            .read_line(&mut column_str)
            .expect("Failed to read input");

        let column: usize = match column_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if column > grid[0].len() - 1 {
            println!("Invalid column number. Please enter a number between 0 and {}.", grid[0].len() - 1);
            continue;
        }

        // Find the first free spot in the selected column
        let mut found = false;
        for (_i, row) in grid.iter_mut().enumerate().rev() {
            if row[column] == ' ' {
                row[column] = current_player;
                found = true;
                break;
            }
        }

        if !found {
            println!("Column is full. Please choose another column.");
            if moves == max_turns {
                println!("The game is a draw!");
                break;
            }
            continue;
        }

        
        for i in 0.. num_cols {
            print!("   {}   ", i);
        }
        println!(" ");
        // Print the grid
        for row in grid.iter() {
            print!("│");
            for element in row.iter() {
                print!("  {}   ", element);
                print!("│");
            }
            println!(" ");
        }

        if check_win(&grid, grid.len(), grid[0].len()) {
            println!(" Player {} Wins!!!! ", current_player);
            break;
        }

        moves += 1;
        if moves == max_turns {
            println!("The game is a draw!");
            break;
        }

        // Alternate between players
        if current_player == '\u{1F534}' {
            player1.push(column);
            current_player = '\u{1F535}';
        } else {
            player2.push(column);
            current_player = '\u{1F534}';
        }
        println!("");
        println!("Player '\u{1F534}':{:?}", player1);
        println!("Player '\u{1F535}':{:?}", player2);

         // Make a copy of the player1 and player2 vectors before they are moved into the GameState struct
        let grid_copy=grid.clone();
        let player1_copy = player1.clone();
        let player2_copy = player2.clone();

         // Save the game state to a file
        let state = GameState{grid:grid_copy, player1:player1_copy, player2:player2_copy, current_player:current_player, max_turns:max_turns, moves:moves};
        let serialized = serde_json::to_string(&state).unwrap();  // serialize the GameState as a JSON string
        let mut file = File::create("state.txt").unwrap();  // create a file
        file.write_all(serialized.as_bytes()).unwrap();

    }




}