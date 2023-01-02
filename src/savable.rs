

// use serde::{Serialize, Deserialize};
// // use std::fs::File;
// // use std::io::prelude::*;
// // use std::io;



#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub grid: Vec<Vec<char>>,
    pub player1: Vec<usize>,
    pub player2: Vec<usize>,
    pub current_player: char,
    pub max_turns: usize,
    pub moves: usize,
    
}



// // fn main() {
// //     let p = Point { x: 10, y: 20 };
// //     let serialized = serde_json::to_string(&p).unwrap();  // serialize the Point as a JSON string

// //     let mut file = File::create("point.txt").unwrap();  // create a file
// //     file.write_all(serialized.as_bytes()).unwrap();  // write the serialized data to the file
// // }

// // pub fn save(s: GameState) {
// //     let serialized = serde_json::to_string(&s).unwrap();  // serialize the Point as a JSON string
// //     let mut file = File::create("point.txt").unwrap();  // create a file
// //     file.write_all(serialized.as_bytes()).unwrap();
// // }


// // pub fn save_to_file(s: String) {
// //     let mut file = File::create("point41.txt");  // create a file
// //     file.write_all(s.as_bytes());  // write the serialized data to the file
// // }

// // pub fn read() {
// //     let file = File::open("point.txt").unwrap();
// //     let p: Point = serde_json::from_reader(file).unwrap();
// //     println!("Point: ({}, {})", p.x, p.y);
// // }