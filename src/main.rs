///////////////////////////////////////////////////////////////////////////////
// Main Application
///////////////////////////////////////////////////////////////////////////////

// General/overall imports:
// imports for Puzzle
use std::fmt; // for debug
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

fn main() {
    // Create a CSV parser that reads data from stdin.
    let rdr = csv::Reader::from_path("./src/puzzles.csv");

    // Loop over each record.
    for result in rdr.unwrap().records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("Expects valid CSV.");
        
        // DEBUG ONLY
        for field in &record {
            print!("{:?},", field);
        }
        print!("\n");

        // converts from single string into array of bytes, then from bytes to base values
        // (48 is constant to convert 0-9 numbers from ascii value)
        let byte_vector = record[1].as_bytes();
        let mut final_vector: Vec<u8> = vec![0; 81];
        for (i, val) in byte_vector.iter().enumerate() {
            final_vector[i] = *val - 48;
        }
        println!("final vector: {:?}", final_vector);

    }
}