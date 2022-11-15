///////////////////////////////////////////////////////////////////////////////
// Main Application
///////////////////////////////////////////////////////////////////////////////

use std::error::Error;
use std::fs::File;

// filepath on run: Ok("/home/figure8/Documents/projects/rust-tools/rusty-sudoku")
// ./src/puzzles.csv

// RETURNING "RESULT" structs
// more info here: https://rust-classes.com/chapter_3_3.html
// Returns either Ok or Err (so a binary worked/didn't work)
// use ".unwrap()" to get the actual value if things went OK

/* 
fn load_puzzles(filepath: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filepath)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
*/

struct Puzzle {
    unsolved: Vec<u8>,
    solved: Vec<u8>
}

fn main() {
    // Create a CSV parser that reads data from stdin.
    let rdr = csv::Reader::from_path("./src/puzzles.csv");
    // Loop over each record.
    for result in rdr.unwrap().records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("Expects valid CSV.");
        for field in &record {
            println!("{:?}", field);
        }
        // Print a debug version of the record.
    }
}