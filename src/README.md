# Rusty-Sudoku

Solves sudokus using Rust. Pretty much what it says on the tin.

Puzzles can come from the user, or from `puzzles.csv`, an attached CSV of puzzles and their solutions.

Currently working on a visualization for the solver algorithim-- basically it reduces a 3D space to a 2D solution grid (which sounds a lot fancier than it is, I promise).

### References / What I Learned and Where:

CSV Parsing: 
https://docs.rs/csv/latest/csv/
https://docs.rs/csv/latest/csv/tutorial/index.html

Working with Results (the type, not the concept) and Error Handling:
https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html
https://rust-classes.com/chapter_3_3.html
