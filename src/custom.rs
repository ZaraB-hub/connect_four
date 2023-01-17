use std::io;

use crate::logic;

pub fn run() {
    loop {
        println!("Enter the number of rows:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let row: usize = input.trim().parse().expect("Invalid input");

        println!("Enter the number of columns:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let col: usize = input.trim().parse().expect("Invalid input");

        if (row as i32 - col as i32).abs() > 2 {
            println!(
                "The difference between the number of rows and columns must not be more than 2"
            );
        } else if row < 6 || col < 7 {
            println!("Board cannot be smaller than 6x7");
        } else {
            logic::run(row, col);
            break;
        }
    }
}
