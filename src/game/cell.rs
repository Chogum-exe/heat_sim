#[derive(Copy, Clone)]
pub struct Cell {
    pub temp: i32,
    n_temp: i32,
    pub x: f32,
    pub y: f32
}

impl Cell {
    pub fn heat(&self, cells: &mut Vec<Cell>) {
        if self.temp != 2 { return () }
        for cell in cells {
            if (cell.x - self.x) as u32 == 1 || cell.x == self.x
            && (cell.y - self.y) as u32 == 1 || cell.y == self.y
            && cell.temp != 2
            && !(cell.x == self.x && cell.y == self.y)
            {
                cell.n_temp += 1;
            }
        }
    }
    pub fn update(&mut self) {
        self.temp = self.n_temp
    }
}

pub fn new(x: f32, y: f32) -> Cell {
    Cell {
        x,
        y,
        temp: 0,
        n_temp: 0
    }
}