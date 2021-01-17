// Main file for sudoku solver
// serves as an entry point for the MVC framework
// (or at least my sloppy version of it)

// constant for the size of the sudoku puzzle
// e.g. 9x9 or whatever

use std::collections::HashMap;

static SUDOKU_SIZE : usize = 9; // needs to be a usize to be itterable

struct SudokuPuzzle {
    grid_squares : Vec<Vec<GridUnit>>,
    row_final_status : Vec<Vec<bool>>,
    col_final_status : Vec<Vec<bool>>,
    box_final_status : Vec<Vec<bool>>
}

#[derive(Clone)]
struct GridUnit {
    solutions : Vec<usize>,
    is_final : bool,
    row_id: usize,
    col_id: usize,
    box_id: usize
}

fn main() {

    // websudoku.com/images/example-steps.html
    // CURRENT ISSUE: trims options correctly, but doesn't affirm single remaining
    // options for solutions based on solution space. See step one on guided walkthrough
    // for exmaple of needed "affirmative" logic.

    let mut test_input : Vec<Vec<usize>> =
        vec![vec![0,0,0,0,0,0,6,8,0],
             vec![0,0,0,0,7,3,0,0,9],
             vec![3,0,9,0,0,0,0,4,5],
             vec![4,9,0,0,0,0,0,0,0],
             vec![8,0,3,0,5,0,9,0,2],
             vec![0,0,0,0,0,0,0,3,6],
             vec![9,6,0,0,0,0,3,0,8],
             vec![7,0,0,6,8,0,0,0,0],
             vec![0,2,8,0,0,0,0,0,0]];

    // ---------------------------------
    // INITALIZATION
    // ---------------------------------

    // Loading the solution space with the input/initial conditions
    let mut solution_space = Vec::new();
    for x in 0..SUDOKU_SIZE {
        let mut solution_space_sub = Vec::new();
            for y in 0..SUDOKU_SIZE {
                let box_row : usize = (x as usize) / 3;
                let box_col : usize = (y as usize) / 3;
                let box_num : usize = (box_row * 3) + box_col;
                let mut single_grid_obj = GridUnit{solutions : vec![],
                                                    is_final : false,
                                                    row_id : x,
                                                    col_id : y,
                                                    box_id : box_num};
                solution_space_sub.push(single_grid_obj);
            }
        solution_space.push(solution_space_sub);
    }

    // filling the 9x9 collection with information
    // (from what can be told from inital conditions)
    for x in 0..SUDOKU_SIZE {
        for y in 0..SUDOKU_SIZE {
            if test_input[x][y] != 0 {
                solution_space[x][y].solutions.push(test_input[x][y]);
                solution_space[x][y].is_final = true;
            } else {
                let filler : Vec<usize> = (1..=9).collect(); // might need to be mut for solve process?
                solution_space[x][y].solutions.extend(filler);
            }
        }
    }

    let mut current_puzzle = SudokuPuzzle{grid_squares : solution_space,
                                         row_final_status : vec![vec![false;9]; 9],
                                         col_final_status : vec![vec![false;9]; 9],
                                         box_final_status : vec![vec![false;9]; 9]};

    // setting final markers
    for x in 0..SUDOKU_SIZE {
        for y in 0.. SUDOKU_SIZE {
            if current_puzzle.grid_squares[x][y].is_final == true {

                // get current items
                let curr_num : usize = current_puzzle.grid_squares[x][y].solutions[0];
                let curr_row : usize = current_puzzle.grid_squares[x][y].row_id;
                let curr_col : usize = current_puzzle.grid_squares[x][y].col_id;
                let curr_box : usize = current_puzzle.grid_squares[x][y].box_id;

                current_puzzle.row_final_status[curr_row][(curr_num-1)] = true;
                current_puzzle.col_final_status[curr_col][(curr_num-1)] = true;
                current_puzzle.box_final_status[curr_box][(curr_num-1)] = true;

            }
        }
    }

    // ---------------------------------
    // SOLUTION PROCESS
    // ---------------------------------
    // variable for the solution state of the puzzle,
    // not just the individual grid items
    let mut iter_num : u64 = 0;
    let mut solved : bool = false;

    // hash map generation process for tracking what values are still needed where
    let mut row_hashes = Vec::new();
    let dummy_val : usize = 0;
    for r in 0..SUDOKU_SIZE {
        // if we're missing any values in the row
        let mut row_hash = HashMap::new();
        for h in 1..SUDOKU_SIZE+1 {
            row_hash.insert(h.to_string(), dummy_val);
        }
        row_hashes.push(row_hash);
    }

    let mut col_hashes = Vec::new();
    for r in 0..SUDOKU_SIZE {
        // if we're missing any values in the row
        let mut col_hash = HashMap::new();
        for h in 1..SUDOKU_SIZE+1 {
            col_hash.insert(h.to_string(), dummy_val);
        }
        col_hashes.push(col_hash);
    }

    let mut box_hashes = Vec::new();
    for r in 0..SUDOKU_SIZE {
        // if we're missing any values in the row
        let mut box_hash = HashMap::new();
        for h in 1..SUDOKU_SIZE+1 {
            box_hash.insert(h.to_string(), dummy_val);
        }
        box_hashes.push(box_hash);
    }

    /*
    println!("{:?}", row_hashes[0]);
    println!("{:?}", col_hashes[0]);
    println!("{:?}", box_hashes[0]);
    */

    while !solved {

        // ---------------------------------
        // SOLUTION PROCESS: REDUCTION
        // ---------------------------------
        for x in 0..SUDOKU_SIZE {
            for y in 0.. SUDOKU_SIZE {

                if current_puzzle.grid_squares[x][y].is_final == true {

                    // get current items
                    let mut curr_num : usize = current_puzzle.grid_squares[x][y].solutions[0];
                    let mut curr_row : usize = current_puzzle.grid_squares[x][y].row_id;
                    let mut curr_col : usize = current_puzzle.grid_squares[x][y].col_id;
                    let mut curr_box : usize = current_puzzle.grid_squares[x][y].box_id;

                    // pass 'is final' data over to the hash map tracker
                    row_hashes[curr_row].insert(curr_num.to_string(), 1);
                    col_hashes[curr_col].insert(curr_num.to_string(), 1);
                    box_hashes[curr_box].insert(curr_num.to_string(), 1);

                    // row removal
                    for c in 0..SUDOKU_SIZE {
                        if c != curr_col {
                            current_puzzle.grid_squares[x][c].solutions.retain(|&x| x != curr_num);
                        }
                    }

                    // col removal
                    for r in 0..SUDOKU_SIZE {
                        if r != curr_row {
                            current_puzzle.grid_squares[r][y].solutions.retain(|&x| x != curr_num);
                        }
                    }

                    // box removal
                    for xx in 0..SUDOKU_SIZE {
                        for yy in 0..SUDOKU_SIZE {
                            let mut temp_box_id : usize = current_puzzle.grid_squares[xx][yy].box_id;
                            if temp_box_id == curr_box && !(curr_row == xx && curr_col == yy) {
                                current_puzzle.grid_squares[xx][yy].solutions.retain(|&x| x != curr_num);
                            }
                        }
                    }
                }
            }
        }

        // ---------------------------------
        // SOLUTION PROCESS: AFFIRMATION
        // ---------------------------------

        // for each row
        for r in 0..SUDOKU_SIZE {
            for c in 1..SUDOKU_SIZE {

            }
        }




        // ---------------------------------
        // DEBUG: END AFTER SET ITTERATION
        // ---------------------------------

        println!("{:?}", iter_num);
        if iter_num < 10 {
            iter_num = iter_num + 1;
        } else {
            break;
        }
    }

    println!("Finished: {0} after {1} iterations.", solved, iter_num);
}
