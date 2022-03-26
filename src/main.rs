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

pub fn load_puzzle(filepath : String, difficuly : String) -> SudokuPuzzle {

    // read in a random puzzle with appropriate difficulty from the file storage
    // "puzzles.csv"

    let mut rdr = csv::Reader::from_reader(filepath);
    for line in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }


}

fn main() {

    val = true;

    if val {
        let csv_path : String = "puzzles.csv";
        let diff : String = "EASY";
        load_puzzle(csv_path, diff);
        return;
    } else {

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
                    let mut filler : Vec<usize> = (1..=9).collect(); // might need to be mut for solve process?
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

            // update final markers
            for x in 0..SUDOKU_SIZE {
                for y in 0.. SUDOKU_SIZE {
                    if current_puzzle.grid_squares[x][y].solutions.len() == 1 {

                        // get current items
                        let curr_num : usize = current_puzzle.grid_squares[x][y].solutions[0];
                        let curr_row : usize = current_puzzle.grid_squares[x][y].row_id;
                        let curr_col : usize = current_puzzle.grid_squares[x][y].col_id;
                        let curr_box : usize = current_puzzle.grid_squares[x][y].box_id;

                        current_puzzle.grid_squares[x][y].is_final = true;
                        current_puzzle.row_final_status[curr_row][(curr_num-1)] = true;
                        current_puzzle.col_final_status[curr_col][(curr_num-1)] = true;
                        current_puzzle.box_final_status[curr_box][(curr_num-1)] = true;

                    }
                }
            }

            // TODO: get rid of row/col/box hash tracking
            //       add logic to update the is_final tracking before affirmation step

            // ---------------------------------
            // SOLUTION PROCESS: AFFIRMATION
            // ---------------------------------

            // for each row
            for r in 0..SUDOKU_SIZE {
                let mut temp_hist = HashMap::new();
                for c in 1..SUDOKU_SIZE {
                    // iterate through solutions for each square and append
                    let temp_vec = &current_puzzle.grid_squares[r][c].solutions;
                    println!("{:?}", temp_vec);
                    for x in  temp_vec {
                        *temp_hist.entry(x.to_string()).or_insert(0) += 1;
                    }
                }

                // at this point, we have a hash map of all the possible values that can be entered
                // into grid square entities for each row.
                //
                // the "affirmation" step means that if there is only one possible place for a number
                // in a row/col/box and the number has not yet been placed, we need to place it now
                // ("affirm" the placement)

                println!("Row final status: {:?}", current_puzzle.row_final_status[r]);

                for n in 1..SUDOKU_SIZE+1 {
                    println!("Row number: {0} checking number {1}", r, n);
                    if temp_hist[&n.to_string()] == 1 && current_puzzle.row_final_status[r][n-1] == false {
                        println!("-----------------");
                        println!("FOUND ONE!");
                        println!("Hash map collection: {:?}", temp_hist);
                        println!("Current status vector: {:?}", current_puzzle.row_final_status[r]);
                        println!("So where is it...?");
                        for t in 0..SUDOKU_SIZE {
                            println!("{:?}", current_puzzle.grid_squares[r][t].solutions);
                        }
                        println!("-----------------")

                        // update the final status tracking to reflect changes
                    }
                }
            }

            // ---------------------------------
            // DEBUG: END AFTER SET ITTERATION
            // ---------------------------------

            println!("{:?}", iter_num);
            if iter_num < 1 {
                iter_num = iter_num + 1;
            } else {
                break;
            }
        }

        println!("Finished: {0} after {1} iterations.", solved, iter_num);

    }
}
