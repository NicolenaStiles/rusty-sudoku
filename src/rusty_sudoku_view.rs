// View printer for Rust-based sudoku solver
// (Assumes you're using a terminal because I'm still pretty basic)

mod rusty_sudoku_model;

pub fn print_sudoku_puzzle(input : Puzzle ) {
    println!("{:?}", input.initial_state);
}
