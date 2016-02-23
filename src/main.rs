extern crate rand;

use std::env;
use std::sync::{Arc, Mutex};
use std::str::FromStr;

use grid::Grid;

mod cell;
mod grid;
mod seeds;

fn main() {

    let seed = env::args().nth(1).map(|s|
        seeds::named(&s).expect("Invalid seed name! Valid seeds are random or gosper_glider")
    ).unwrap_or(seeds::gosper_glider);

    let max_gen = env::args().nth(2).map(|s|
        u32::from_str(&s).expect("Only numbers!")
    ).unwrap_or(1);

    let width = env::args().nth(3).map(|s|
        i16::from_str(&s).expect("Only numbers!")
    ).unwrap_or(100);

    let height = env::args().nth(4).map(|s|
        i16::from_str(&s).expect("Only numbers!")
    ).unwrap_or(50);

    let grid = Arc::new(Mutex::new(Grid::new(seed, width, height)));
    let mut gen = 1;

    println!("{}",max_gen);

    grid.lock().unwrap().draw();

    for _ in 0..max_gen {
        if gen == max_gen {
            return;
        }
        gen += 1;
        grid.lock().unwrap().update();
        grid.lock().unwrap().draw();
    }
}
