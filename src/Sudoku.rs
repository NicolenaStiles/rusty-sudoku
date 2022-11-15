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

struct Puzzle {
    idx: u32,
    difficulty: Difficulty,
    initial: Vec<u8>,
    is_started: bool,
    current_state: Vec<u8>,
    solution: Vec<u8>,    
}

// implementation of Puzzle
// includes:
// - debug status and printing
// TODO: add instantiaion function? Feels like that isn't needed...?
impl Puzzle {
    fn print_status(&mut self) {
        println!("Puzzle #: {:?}", self.idx);
        println!("Difficulty: {:?}", self.difficulty);
        println!("Initial: {:?}", self.initial);
        println!("Is started? {:?}", self.is_started);
        println!("Current State: {:?}", self.current_state);
        println!("Solution: {:?}", self.solution);
    }
}

// Converts single string into vector of u8 ints
fn string_to_int_vec() {

}