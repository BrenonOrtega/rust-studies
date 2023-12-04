use crate::frame::Frame;
use crate::{NUMBER_ROWS, NUMBER_COLUMNS};
use crate::render::Drawable;

pub struct Player {
    x_index: usize,
    y_index: usize
}

impl Player {
    pub fn new() -> Player {
        Player { 
            x_index: NUMBER_COLUMNS / 2,
            y_index: NUMBER_ROWS - 1, 
        }
    }

    pub fn move_left(&mut self) -> () {
        if self.x_index == 0 { return }

        self.x_index -= 1;
    }

    pub fn move_right(&mut self) -> () {
        if self.x_index == NUMBER_COLUMNS - 1 { return }

        self.x_index += 1;
    }

    pub fn shoot(&self) {
        println!("tried to shoot - not implemented");
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x_index][self.y_index] = "A";
    }
}