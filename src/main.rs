// Main file for sudoku solver
// serves as an entry point for the MVC framework
// (or at least my sloppy version of it)

struct SudokuPuzzle {
    grid_squares : Vec<Vec<GridUnit>>,
    row_final_status : Vec<bool>,
    col_final_status : Vec<bool>,
    box_final_status : Vec<bool>
}

#[derive(Clone)]
struct GridUnit {
    solutions : Vec<u8>,
    is_final : bool,
    row_id: u8,
    col_id: u8,
    box_id: u8,

}

fn main() {

    // websudoku.com/images/example-steps.html
    // CURRENT ISSUE: trims options correctly, but doesn't affirm single remaining
    // options for solutions based on solution space. See step one on guided walkthrough
    // for exmaple of needed "affirmative" logic.

    let mut test_input : Vec<Vec<u8>> =
        vec![vec![0,0,0,0,0,0,6,8,0],
             vec![0,0,0,0,7,3,0,0,9],
             vec![3,0,9,0,0,0,0,4,5],
             vec![4,9,0,0,0,0,0,0,0],
             vec![8,0,3,0,5,0,9,0,2],
             vec![0,0,0,0,0,0,0,3,6],
             vec![9,6,0,0,0,0,3,0,8],
             vec![7,0,0,6,8,0,0,0,0],
             vec![0,2,8,0,0,0,0,0,0]];

}
