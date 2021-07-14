use ggez::{Context, GameResult, timer};
use ggez::event::EventHandler;
use ggez::graphics::*;

mod display;
mod cell;

pub struct Game {
    // Your state here...
    cells: Vec<cell::Cell>,
    tile_s: f32
}

impl Game {
    pub fn new(width: f32, height: f32, tile_s: f32) -> Game {
        // Load/create resources here: images, fonts, sounds, etc.
        let mut list = Vec::new();
        for i in 0..(height * width / (tile_s*tile_s)) as i32 {
            let i = i as f32;
            list.push(cell::new(i % (width / tile_s), i / width / tile_s))
        }
        println!("{}", list.len());
        Game {
            cells: list,
            tile_s
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        timer::sleep(std::time::Duration::from_secs(2));
        for i in 0..self.cells.len() {
            self.cells[i].clone().heat(&mut self.cells);
        }
        for i in 0..self.cells.len() {
            self.cells[i].update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, BLACK);
        // Draw code here...
        for cell in &self.cells {
            let color: (f32, f32, f32, f32);
            if cell.temp == 0 {
                color = (250.0, 250.0, 0.0, 1.0)
            }
            else if cell.temp == 1 {
                color = (250.0, 125.0, 0.0, 1.0)
            }
            else if cell.temp == 2 {
                color = (250.0, 0.0, 0.0, 1.0)
            } else { color = (0.0, 250.0, 0.0, 1.0)}
            display::rect(
                cell.x * self.tile_s, cell.y * self.tile_s,
                self.tile_s,
                self.tile_s,
                color,
                false, 0.0,
                ctx
            );
            display::rect(
                cell.x * self.tile_s, cell.y * self.tile_s,
                self.tile_s,
                self.tile_s,
                (0.0, 0.0, 0.0, 1.0),
                true, 2.5,
                ctx
            );
        }
        present(ctx)
    }
}