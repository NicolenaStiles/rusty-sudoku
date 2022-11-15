///////////////////////////////////////////////////////////////////////////////
// Sudoku Crate
///////////////////////////////////////////////////////////////////////////////
// Contains:
// - Puzzle
///////////////////////////////////////////////////////////////////////////////

// General/overall imports:
// imports for Puzzle Selection
use std::fmt; // for debug

// for random seed/site generation
use rand::Rng;

///////////////////////////////////////////////////////////////////////////////
// SudokuPuzzle
///////////////////////////////////////////////////////////////////////////////
// Data structure that holds all the info about Sudoku puzzle.
///////////////////////////////////////////////////////////////////////////////
// Types and Defaults:
// - idx: u32
// - difficulty: Difficulty = undefined
// - initial: Vec<u8>
// - is_started: bool
// - current_state: Vec<u8>
// - solution: Vec<u8>  
///////////////////////////////////////////////////////////////////////////////

// field Difficulty for Puzzle
#[derive(Copy,Clone)]
enum Difficulty {
    UNDEFINED,
    EASY,
    MEDIUM,
    HARD
}

// debug implementation for UnitType enum
impl fmt::Debug for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Difficulty::UNDEFINED => write!(f, "Undefined"),
            Difficulty::EASY => write!(f, "Easy"),
            Difficulty::MEDIUM => write!(f, "Medium"),
            Difficulty::HARD => write!(f, "Hard"),        
        }
    }
}

pub struct Puzzle {
    idx: u32,
    difficulty: Difficulty,
    initial: Vec<u8>,
    is_started: bool,
    current_state: Vec<u8>,
    solution: Vec<u8>,    
}

// implementation of Puzzle
// includes:
// - load random puzzle
// - load puzzle from idx
// - debug status and printing
// TODO: add instantiaion function? Feels like that isn't needed...?
impl Puzzle {

    // this is kinda nonsense tbh but I think it has to be here
    pub fn new() -> Self {
        Puzzle {
            idx: 0,
            difficulty: Difficulty::UNDEFINED,
            initial: vec![0; 81],
            is_started: false,
            current_state: vec![0; 81],
            solution: vec![0; 81]
        }
    }

    pub fn load_random_puzzle(&mut self, filepath: String) {
        println!("{:?}", filepath);
        println!("Not implemented!");
    }

    pub fn load_puzzle_from_idx(&mut self, filepath: String, idx: u32) {
        // Spin up CSV parser
        let rdr = csv::Reader::from_path(filepath);
        // Each line is an entry/puzzle
        for entry in rdr.unwrap().records() { 
            let record = entry.expect("Expects valid CSV... whoops!");
            // If the idx matches the one we're looking for, we're good!
            // Otherwise... keep going.
            let idx_field: u32 = record[0].parse().unwrap();
            if idx_field == idx {
                println!("Found it!");
                self.idx = idx_field;
            } else {
                println!("Not it!");
            }
        }
    }

    pub fn print_status(&mut self) {
        println!("Puzzle #: {:?}", self.idx);
        println!("Difficulty: {:?}", self.difficulty);
        println!("Initial: {:?}", self.initial);
        println!("Is started? {:?}", self.is_started);
        println!("Current State: {:?}", self.current_state);
        println!("Solution: {:?}", self.solution);
    }
}

// Converts single string into vector of u8 ints
/* 
fn string_to_int_vec() {

}
*/