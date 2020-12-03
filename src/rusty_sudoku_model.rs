// Model file for the rust sudoku solver

// todo: constrain length of vectors?
pub struct Puzzle {
    pub initial_state : Vec<u8>,
    pub current_state : Vec<u8>,
    pub solution : Vec<u8>
}

impl Puzzle {
    pub fn print_init_state(&mut self) {
        println!("{:?}",self.initial_state);
    }

    pub fn print_curr_state(&mut self) {
        println!("{:?}",self.current_state);
    }

    pub fn print_soln_state(&mut self) {
        println!("{:?}",self.solution);
    }
}
