pub mod guess;
pub mod log;
pub mod generic_func;
pub mod counter;

// use crate::guess::guess_game;
// use crate::log::run_logger_example;
// use crate::generic_func::run_generic_example;
use crate::counter::run_counter_example;

fn main() {
    // guess_game();
    // run_logger_example();
    // run_generic_example();
    run_counter_example();
}

