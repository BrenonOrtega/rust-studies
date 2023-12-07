use std::time::Duration;

use crate::frame::{Frame, Drawable};
use crate::render::Updatable;
use crate::shot::Shot;
use crate::{NUMBER_ROWS, NUMBER_COLUMNS};

pub struct Player {
    x_index: usize,
    y_index: usize,
    shots: Vec<Shot>
}

impl Player {
    pub fn new() -> Player {
        Player { 
            x_index: NUMBER_COLUMNS / 2,
            y_index: NUMBER_ROWS - 1, 
            shots: Vec::new()
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

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 3 {
            self.shots.push(Shot::new(self.x_index, self.y_index));
            
            true
        } 
        else {    
            false
        }
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x_index][self.y_index] = "A";

        self.shots.iter().for_each(|shot: &Shot| {
            shot.draw(frame);
        });
    }
}

impl Updatable for Player {
    fn update(&mut self, delta: &Duration) {
        self.shots.iter_mut()
            .for_each(|shot| shot.update(*delta));
        
        self.shots.retain(|shot| !shot.dead());
    }
}