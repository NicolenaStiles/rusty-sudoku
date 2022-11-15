///////////////////////////////////////////////////////////////////////////////
// Main Application
///////////////////////////////////////////////////////////////////////////////

mod Sudoku;

fn main() {
    let csv_filepath: String  = String::from("./src/csv_test.csv");
    let test_idx : u32 = 3;
    let mut test_puzzle : Sudoku::Puzzle = Sudoku::Puzzle::new();
    test_puzzle.load_puzzle_from_idx(csv_filepath,test_idx);
    test_puzzle.print_status();
}
