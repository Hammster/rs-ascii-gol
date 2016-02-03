use cell::Cell;
use seeds::Seed;

pub struct Grid {
    pub cells: Vec<Cell>,
    pub width: i16,
    pub height: i16,
}

impl Grid {
    pub fn new(seed: Seed, width: i16, height: i16) -> Grid {
        let mut cells = Vec::new();

        for y in 0..height {
            for x in 0..width {
                cells.push(Cell {
                    x: x,
                    y: y,
                    neighbours: [
                        (x-1, y-1), (x, y-1), (x+1, y-1),
                        (x-1, y  ),           (x+1, y  ),
                        (x-1, y+1), (x, y+1), (x+1, y+1)
                    ].iter().map(|n| coords_to_index(*n, width, height)).collect(),
                    alive: seed(x, y),
                });
            }
        }
        Grid{ cells: cells, width: width, height: height }
    }

    pub fn update(&mut self) {
        let mut alive_neighbours = Vec::new();
        for cell in self.cells.iter() {
            alive_neighbours.push(cell.neighbours.iter().filter(|n| self.cells[**n].alive).count())
        }

        for (cell, cell_alive_neighbours) in self.cells.iter_mut().zip(alive_neighbours.iter()) {
            cell.update(*cell_alive_neighbours)
        }
    }

    pub fn draw(&mut self) {
        print!("╔");
        for _ in 0..self.width {
            print!("═");
        }
        println!("╗");
        for cell in self.cells.iter() {
            if cell.x == 0 {
                print!("║");
            }
            if cell.alive == true {
                print!("☼");
            } else {
                print!(" ");
            }
            if cell.x == self.width-1 {
                println!("║");
            }
        }
        print!("╚");
        for _ in 0..self.width {
            print!("═");
        }
        println!("╝");
    }
}

fn coords_to_index(coords: (i16, i16), grid_width: i16, grid_height: i16) -> usize {
    let (x, y) = coords;
    let x_wrapped = (x + grid_width) % grid_width;
    let y_wrapped = (y + grid_height) % grid_height;
    (x_wrapped + (y_wrapped * grid_width)) as usize
}
