///////////////////////////////////////////////////////////////////////////////
// Sudoku Crate
///////////////////////////////////////////////////////////////////////////////
// Contains:
// - Puzzle
///////////////////////////////////////////////////////////////////////////////

// General/overall imports:
// imports for Puzzle Selection
use std::fmt; // for debug

// for random puzzle selection
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
pub enum Difficulty {
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
// - load puzzle from idx
// - debug status and printing
// TODO: Load random puzzle based on difficulty
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

    pub fn load_puzzle_from_difficulty(&mut self, filepath: String, difficulty: Difficulty) {
        // Vector of potential idx of unsolved & desired difficulty
        let mut diff_string = "";
        match difficulty {
            Difficulty::UNDEFINED =>
                diff_string = "UNDEFINED",
            Difficulty::EASY =>
                diff_string= "EASY",
            Difficulty::MEDIUM =>
                diff_string = "MEDIUM",
            Difficulty:: HARD =>
                diff_string = "HARD"
        }
        let mut puzzle_vects: Vec<u32> = Vec::new();
        // Spin up CSV parser
        let rdr = csv::Reader::from_path(filepath);
        // Each line is an entry/puzzle
        for entry in rdr.unwrap().records() { 
            let record = entry.expect("Expects valid CSV... whoops!");
            // Pull out puzzles that match desired difficulty
            let diff_field: String = record[1].parse().unwrap();
            if diff_field == diff_string {
                let idx_field: u32 = record[0].parse().unwrap();
                puzzle_vects.push(idx_field);
            }
        }
        println!("{:?}", puzzle_vects);
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
                // idx
                self.idx = idx_field;
                // difficulty
                let diff_field : String = record[1].parse().unwrap();
                match diff_field.as_str() {
                    "UNDEFINED" => 
                        self.difficulty = Difficulty::UNDEFINED,
                    "EASY" =>
                        self.difficulty = Difficulty::EASY,
                    "MEDIUM" =>
                        self.difficulty = Difficulty::MEDIUM,
                    "HARD" =>
                        self.difficulty = Difficulty::HARD,
                    _ =>
                        println!("Oops!"),
                }
                // initial
                let intl_bytes = record[2].as_bytes();
                let mut intl_vector: Vec<u8> = vec![0; 81];
                for (i, val) in intl_bytes.iter().enumerate() {
                    intl_vector[i] = *val - 48;
                }
                self.initial = intl_vector;
                // is started
                let is_started : String = record[3].parse().unwrap();
                match is_started.as_str() {
                    "true" =>
                        self.is_started = true,
                    "false" =>
                        self.is_started = false,
                    _ => 
                        println!("Oops!"),
                }
                // current state
                let state_bytes = record[4].as_bytes();
                let mut state_vector: Vec<u8> = vec![0; 81];
                for (i, val) in state_bytes.iter().enumerate() {
                    state_vector[i] = *val - 48;
                }
                self.current_state = state_vector;   
                // solution
                let soln_bytes = record[5].as_bytes();
                let mut soln_vector: Vec<u8> = vec![0; 81];
                for (i, val) in soln_bytes.iter().enumerate() {
                    soln_vector[i] = *val - 48;
                }
                self.solution = soln_vector;                              
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