use std::time::Duration;

use crate::frame::{Frame, Drawable};
use crate::invaders::Invaders;
use crate::render::Updatable;
use crate::shot::Shot;
use crate::{NUMBER_ROWS, NUMBER_COLUMNS, PLAYER_FORM};

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
        if self.shots.len() < 20 {
            self.shots.push(Shot::new(self.x_index, self.y_index));
            
            true
        } 
        else {    
            false
        }
    }

    pub fn has_killed_any_invader(&mut self, invaders: &mut Invaders) -> bool {
        self.shots.iter_mut().any(|shot| {
            let killed = invaders.try_kill_invaders_at(shot.x_index, shot.y_index);

            if killed {
                shot.explode();
            }

            killed
        })
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x_index][self.y_index] = PLAYER_FORM;

        self.shots.iter().for_each(|shot: &Shot| {
            shot.draw(frame);
        });
    }
}

impl Updatable for Player {
    fn update(&mut self, delta: Duration) {
        self.shots.iter_mut()
            .for_each(|shot| shot.update(delta));
        
        self.shots.retain(|shot: &Shot| !shot.dead());
    }
}