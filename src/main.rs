// creates shortcuts to items to reduce repetition of long paths
use crate::guess::guessing;

// tells the compiler to include the code it finds in src/guess.rs
pub mod guess;

// Entry point of a rust program
fn main() {
    guessing::run();
}
