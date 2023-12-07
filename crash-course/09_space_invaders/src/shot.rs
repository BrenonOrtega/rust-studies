use std::time::Duration;

use rusty_time::Timer;

use crate::frame::Drawable;

pub struct Shot {
    x_index: usize,
    y_index: usize,
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

    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);

        if !self.timer.ready || self.exploding { return }

        if self.y_index > 0 {
            self.y_index -= 1;
        }

        self.timer.reset(); 
    }

    pub fn dead(&self) -> bool {
        (self.y_index == 0) || (self.exploding && self.timer.ready)  
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x_index][self.y_index] = "|";
    }
}
