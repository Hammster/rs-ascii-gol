extern crate rand;

use std::env;
use std::sync::{Arc, Mutex};
use std::io;

use grid::Grid;

mod cell;
mod grid;
mod seeds;

fn main() {

    let mut input = String::new();

    let seed = env::args().nth(1).map(|s|
        seeds::named(&s).expect("Invalid seed name! Valid seeds are random or gosper_glider")
    ).unwrap_or(seeds::random);


    let grid = Arc::new(Mutex::new(Grid::new(seed, 100, 50)));
    let mut gen = 1;


    grid.lock().unwrap().draw();

    loop{
        println!("How many generations ?");

        io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

        let trimmed = input.trim();

        match trimmed.parse::<u32>() {
            Ok(i) => {
                for _ in 0..i {
                    gen += 1;

                    grid.lock().unwrap().update();
                    grid.lock().unwrap().draw();
                }
                println!("({}th gen)",gen);
                //grid.lock().unwrap().draw();
            },
            Err(..) => return
        };

    }

}
