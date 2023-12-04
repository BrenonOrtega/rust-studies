use crate::{NUMBER_COLUMNS, NUMBER_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut cols: Vec<Vec<&'static str>> = Vec::with_capacity(NUMBER_COLUMNS);
    
    for _ in 0..NUMBER_COLUMNS {
        let mut col: Vec<&str> = Vec::with_capacity(NUMBER_ROWS);

        for _ in 0..NUMBER_ROWS {
            col.push(" ");
        }
        
        cols.push(col);
    }

    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}