// Main file for sudoku solver
// serves as an entry point for the MVC framework
// (or at least my sloppy version of it)

// constant for the size of the sudoku puzzle
// e.g. 9x9 or whatever

static SUDOKU_SIZE : usize = 9; // needs to be a usize to be itterable

struct SudokuPuzzle {
    grid_squares : Vec<Vec<GridUnit>>,
    row_final_status : Vec<Vec<bool>>,
    col_final_status : Vec<Vec<bool>>,
    box_final_status : Vec<Vec<bool>>
}

#[derive(Clone)]
struct GridUnit {
    solutions : Vec<u8>,
    is_final : bool,
    row_id: u8,
    col_id: u8,
    box_id: u8
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

    // ---------------------------------
    // INITALIZATION
    // ---------------------------------

    // Loading the solution space with the input/initial conditions
    let mut solution_space = Vec::new();
    for x in 0..SUDOKU_SIZE {
        let mut solution_space_sub = Vec::new();
            for y in 0..SUDOKU_SIZE {
                let box_row : u8 = (x as u8) / 3;
                let box_col : u8 = (y as u8) / 3;
                let box_num = (box_row * 3) + box_col;
                let mut single_grid_obj = GridUnit{solutions : vec![],
                                                    is_final : false,
                                                    row_id : x as u8,
                                                    col_id : y as u8,
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
                let mut filler : Vec<u8> = (1..=9).collect();
                solution_space[x][y].solutions.extend(filler);
            }
        }
    }

    let mut current_puzzle = SudokuPuzzle{grid_squares : solution_space,
                                         row_final_status : vec![vec![false;9]; 9],
                                         col_final_status : vec![vec![false;9]; 9],
                                         box_final_status : vec![vec![false;9]; 9]};

    for row in 0..SUDOKU_SIZE{
        for col in 0..SUDOKU_SIZE {
            if current_puzzle.grid_squares[row][col].is_final == true {
                println!("{:?}", current_puzzle.grid_squares[row][col].solutions);
                println!("{:?}", current_puzzle.grid_squares[row][col].row_id);
                println!("{:?}", current_puzzle.grid_squares[row][col].col_id);
                println!("{:?}", current_puzzle.grid_squares[row][col].box_id);
                println!("---------------");
            }
        }
    }

    println!("=====================");


    println!("{:?}", current_puzzle.row_final_status[0]);
    // setting final markers
    for x in 0..SUDOKU_SIZE {
        for y in 0.. SUDOKU_SIZE {
            if current_puzzle.grid_squares[x][y].is_final == true {

                // get current items
                let curr_num : u8 = current_puzzle.grid_squares[x][y].solutions[0];

                let curr_row : u8 = current_puzzle.grid_squares[x][y].row_id;
                let curr_col : u8 = current_puzzle.grid_squares[x][y].col_id;
                let curr_box : u8 = current_puzzle.grid_squares[x][y].box_id;

                println!("val: {:0}, row: {:1}, col: {:2}, box: {:3}", curr_num,
                                                                       curr_row,
                                                                       curr_col,
                                                                       curr_box);


                current_puzzle.row_final_status[curr_row as usize][(curr_num-1) as usize] = true;

                /*
                current_puzzle.row_final_status[(curr_num-1) as usize] = true;
                current_puzzle.col_final_status[(curr_num-1) as usize] = true;
                current_puzzle.box_final_status[(curr_num-1) as usize] = true;
                */
            }
        }
    }

    println!("{:?}", current_puzzle.row_final_status[0]);

    // ---------------------------------
    // SOLUTION PROCESS
    // ---------------------------------
    // variable for the solution state of the puzzle,
    // not just the individual grid items
    let mut iter_num : u64 = 0;
    let mut solved : bool = false;

    /*

    while !solved {

        // ---------------------------------
        // SOLUTION PROCESS: REDUCTION
        // ---------------------------------
        for x in 0..SUDOKU_SIZE {
            for y in 0.. SUDOKU_SIZE {
                if solution_space[x][y].is_final == true {

                    // get current items
                    let mut curr_num : u8 = solution_space[x][y].solutions[0];
                    let mut curr_row : u8 = solution_space[x][y].row_id;
                    let mut curr_col : u8 = solution_space[x][y].col_id;
                    let mut curr_box : u8 = solution_space[x][y].box_id;

                    // row removal
                    for c in 0..SUDOKU_SIZE {
                        if c != curr_col {
                            solution_space[curr_row][c].solutions.retain(|&x| x != curr_num);
                        }
                    }

                    // col removal
                    for r in 0..SUDOKU_SIZE {
                        if r != curr_row {
                            solution_space[r][curr_col].solutions.retain(|&x| x != curr_num);
                        }
                    }

                    // box removal
                    for xx in 0..SUDOKU_SIZE {
                        for yy in 0..SUDOKU_SIZE {
                            let mut temp_box_id : u8 = solution_space[xx][yy].box_id;
                            if temp_box_id == box_id && !(row_id == xx && col_id == yy) {
                                solution_space[xx][yy].solutions.retain(|&x| x != curr_num);
                            }
                        }
                    }
                }
            }
        }
    }

    */
}
