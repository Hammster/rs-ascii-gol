use std::fmt;

pub struct Cell {
    pub x: i16,
    pub y: i16,
    pub neighbours: Vec<usize>,
    pub alive: bool,
}

impl Cell {
    pub fn update(&mut self, alive_neighbours: usize) {
        self.alive = match (self.alive, alive_neighbours) {
            (false, 3) => true,
            (true, 2) => true,
            (true, 3) => true,
            _ => false
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cell {{ x: {}, y: {}, alive: {} }}", self.x, self.y, self.alive)
    }
}
