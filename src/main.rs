// Main file for sudoku solver
// serves as an entry point for the MVC framework
// (or at least my sloppy version of it)

extern crate bitflags;
mod rusty_sudoku_model;

// DEBUG ONLY
// function to print sudoku grids
// takes in immutable slice of the full on vector
fn pretty_print_grid(grid_numbers : &[u8]) {

   // slice the vector and setting up range bounds
   let sudoku_size : usize = 9;
   let mut curr_idx : usize = 1;
   let mut low_idx : usize;

   while curr_idx <= grid_numbers.len() {
        // printing the current line
        if curr_idx % sudoku_size == 0 {
            low_idx = curr_idx - sudoku_size;
            println!("{:?}", &grid_numbers[low_idx..curr_idx]);
            curr_idx = curr_idx + 1;
        }
        else {
            curr_idx = curr_idx + 1;
        }
   }
}

fn main() {

    // test entry array for debugging
    // (make it a string since that's how we read input)
    let mut test_input : Vec<u8> =
        vec![2,7,8,1,0,0,0,0,3,
             6,0,0,3,8,0,0,5,1,
             0,1,0,7,4,0,0,2,0,
             1,0,5,0,7,0,2,0,0,
             3,0,0,8,2,4,1,0,0,
             0,0,4,0,0,0,9,3,0,
             0,5,1,0,0,8,4,7,0,
             0,0,0,0,0,7,0,9,8,
             0,8,6,0,5,9,0,0,0];

    let mut current_input : Vec<u8> =
        vec![2,7,8,1,0,0,0,0,3,
             6,0,0,3,8,0,0,5,1,
             0,1,0,7,4,0,0,2,0,
             1,0,5,0,7,0,2,0,0,
             3,0,0,8,2,4,1,0,0,
             0,0,4,0,0,0,9,3,0,
             0,5,1,0,0,8,4,7,0,
             0,0,0,0,0,7,0,9,8,
             0,8,6,0,5,9,0,0,0];


    let mut test_solution : Vec<u8> =
        vec![2,7,8,1,9,5,6,4,3,
             6,4,9,3,8,2,7,5,1,
             5,1,3,7,4,6,8,2,9,
             1,6,5,9,7,3,2,8,4,
             3,9,7,8,2,4,1,6,5,
             8,2,4,5,6,1,9,3,7,
             9,5,1,2,3,8,4,7,6,
             4,3,2,6,1,7,5,9,8,
             7,8,6,4,5,9,3,1,2];

    /*
    let mut test_puzzle = rusty_sudoku_model::Puzzle{
                            initial_state: test_input,
                            current_state : current_input,
                            solution: test_solution
                            };

    test_puzzle.print_init_state();
    test_puzzle.print_soln_state();
    */

    // testing debug print functionality (it works btw)
    // pretty_print_grid(test_input.as_slice());

    // testing the ssudoku solver process

    // start by declaring 81 bitfields 9 bits long (sucks they can't be 8 bits or anything easy)
    

}
