use rusty_time::Timer;

use crate::frame::Drawable;

pub struct Shot {
    x_index: usize,
    y_index: usize,
    timer: Timer
}

impl Shot {
    pub fn new(x_index: usize, y_index: usize) -> Shot {
        Shot {
            x_index,
            y_index: y_index - 1,
            timer: Timer::from_millis(10)
        }
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x_index][self.y_index] = "|";
    }
}