use std::time::Duration;

use rusty_time::Timer;

use crate::{frame::Drawable, SHOT_FORM, render::Updatable};

pub struct Shot {
    pub x_index: usize,
    pub y_index: usize,
    exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x_index: usize, y_index: usize) -> Shot {
        Shot {
            x_index,
            y_index: y_index - 1,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }

    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || self.y_index == 0
    }

    pub(crate) fn explode(&mut self) {
        self.exploding = true;
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x_index][self.y_index] = if !self.exploding { SHOT_FORM } else { "*" };
    }
}

impl Updatable for Shot {
    fn update(&mut self, delta: Duration) {
         self.timer.update(delta);

        if !self.timer.ready || self.exploding { return }

        if self.y_index > 0 {
            self.y_index -= 1;
        } else {
            self.explode();
        }

        if self.y_index == 0 {
            self.explode();
        }

        self.timer.reset(); 
    }
}