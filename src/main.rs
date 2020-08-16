use crate::automata::runner::Runner;
use clap::{App, Arg, ArgMatches};
use std::time::Duration;

mod automata;

fn args() -> ArgMatches {
    App::new("Game of life")
        .version("0.1.0")
        .author("CodeOverFlow <adrien.bodineau@gmail.com>")
        .about("Game of life in Rust, for training purpose only")
        .arg(
            Arg::with_name("grid_width")
                .long("grid_width")
                .value_name("WIDTH")
                .about("Sets the width of the grid")
                .takes_value(true)
                .default_value("5"),
        )
        .arg(
            Arg::with_name("grid_height")
                .long("grid_height")
                .value_name("HEIGHT")
                .about("Sets the height of the grid")
                .takes_value(true)
                .default_value("5"),
        )
        .arg(
            Arg::with_name("max_steps")
                .long("max_steps")
                .value_name("MAX_STEPS")
                .about("Sets the maximum number of steps in the simulation")
                .takes_value(true)
                .default_value("5"),
        )
        .arg(
            Arg::with_name("step_timer")
                .long("step_timer")
                .value_name("STEP_TIMER")
                .about("Sets the waiting time between two steps, in integer seconds")
                .takes_value(true)
                .default_value("2.0"),
        )
        .get_matches()
}

fn main() {
    let mut max_steps: u32 = 5;
    let mut step_timer = Duration::from_secs_f64(2.0);
    let mut width: u32 = 5;
    let mut height: u32 = 5;

    // PARSE ARGS
    let matches = args();

    if let Some(matches) = matches.value_of("max_steps") {
        if let Ok(value) = matches.parse() {
            max_steps = value;
        }
    }

    if let Some(matches) = matches.value_of("step_timer") {
        if let Ok(value) = matches.parse() {
            step_timer = Duration::from_secs_f64(value);
        }
    }

    if let Some(matches) = matches.value_of("grid_width") {
        if let Ok(value) = matches.parse() {
            width = value;
        }
    }

    if let Some(matches) = matches.value_of("grid_height") {
        if let Ok(value) = matches.parse() {
            height = value;
        }
    }

    let runner = Runner::new(width, height, max_steps, step_timer);
    runner.initialize_grid(vec![
        (1, 1),
        (1, 2),
        (1, 3),
        (2, 1),
        (2, 2),
        (2, 3),
        (3, 1),
        (3, 2),
        (3, 3),
        (4, 1),
        (4, 2),
        (4, 3),
        (5, 1),
        (5, 2),
        (5, 3),
    ]);
    runner.run();
}
