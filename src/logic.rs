use crate::win::check_win;
use std::io;

pub fn run(row: usize, col: usize) {
    let mut grid: Vec<Vec<char>> = vec![vec![' '; col]; row];

    let mut current_player = '\u{1F534}'; // Red player goes first
    let mut player1: Vec<usize> = Vec::new();
    let mut player2: Vec<usize> = Vec::new(); // Vector to store player moves
    let max_turns = col*row; // Maximum number of turns allowed (6 rows * 7 columns)
    let mut moves = 0;

    for i in 0.. col {
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

    loop {
        println!("Enter a column number (0-{}):", col - 1);

        let mut column_str = String::new();
        io::stdin()
            .read_line(&mut column_str)
            .expect("Failed to read input");

        let column: usize = match column_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if column > col - 1 {
            println!("Invalid column number. Please enter a number between 0 and {}.",col-1);
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
            continue;
        }

        //print column numbers above
        for i in 0..=col - 1 {
            print!("   {}   ", i);
        }
        println!(" \n");
        // Print the grid

        for row in grid.iter() {
            
            print!("│");
            for element in row.iter() {
                print!("{:^6}", element);
                print!("│");
            }
            println!(" ");
        }

        if check_win(&grid, row, col) {
            println!(" Player {} Wins!!!! ",current_player);
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
    }
}