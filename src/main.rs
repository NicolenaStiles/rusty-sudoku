///////////////////////////////////////////////////////////////////////////////
// Main Application
///////////////////////////////////////////////////////////////////////////////

use Sudoku::Difficulty;

mod Sudoku;

fn main() {
    // test: load from specific index
    let csv_filepath: String  = String::from("./src/csv_test.csv");
    let test_idx : u32 = 9;
    let mut test_puzzle_1 : Sudoku::Puzzle = Sudoku::Puzzle::new();
    test_puzzle_1.load_puzzle_from_idx(csv_filepath,test_idx);
    test_puzzle_1.print_status();

    // test: load from chosen difficulty
    let csv_filepath: String  = String::from("./src/csv_test.csv");
    let test_difficulty: Difficulty = Difficulty::EASY;
    let mut test_puzzle_2 : Sudoku::Puzzle = Sudoku::Puzzle::new();
    test_puzzle_2.load_puzzle_from_difficulty(csv_filepath, test_difficulty);
}