// Scratchpad for testing out ideas and etc

/*


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

    // ---------------------------------
    // SOLUTION PROCESS: AFFIRMATION
    // ---------------------------------

    // row affirmation
    for r in 0..SUDOKU_SIZE {
        // if we're missing any values in the row
        let mut row_hash = HashMap::new();
        for h in 0..SUDOKU_SIZE {
            row_hash.insert(h.to_string(), None);
        }

        println!("{:?}", row_hash);
        break;

        for c in 0..SUDOKU_SIZE {

        }

    }

    // col affirmation

    // box affirmation

    // DEBUG ONLY
    if iter_num > 10 {
        break;
    } else {
        iter_num = iter_num + 1;
    }

}

// ---------------------------------
// DEBUG: PRINTING PUZZLE STATUS
// ---------------------------------

for xx in 0..SUDOKU_SIZE {
    for yy in 0..SUDOKU_SIZE{
        println!("{0}, {1} = {2}" , xx, yy, current_puzzle.grid_squares[xx][yy].is_final);
    }
}


*/
