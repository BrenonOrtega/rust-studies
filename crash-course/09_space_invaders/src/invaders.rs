use std::{cmp::max, time::Duration};

use rusty_time::Timer;

use crate::{
    frame::{Drawable, Frame},
    render::Updatable,
    NUMBER_COLUMNS, INVADERS_FORMS, NUMBER_ROWS
};

pub struct Invader {
    x_index: usize,
    y_index: usize,
}

impl Invader {
    pub fn new(x_index: usize, y_index: usize) -> Invader {
        Invader {
            x_index,
            y_index,
        }
    }

    pub fn create_level(level: usize) -> Vec<Invader> {
        let invaders_qty = NUMBER_COLUMNS * level / 3;
        let mut invaders = Vec::with_capacity(invaders_qty);

        let invaders_x_indexes = Invader::get_invaders_indexes(NUMBER_COLUMNS);

        for x_index in invaders_x_indexes {
            for y_index in 0..=level {
                invaders.push(Invader::new(x_index, y_index))
            }
        }

        invaders
    }

    fn get_invaders_indexes(number_columns: usize) -> Vec<usize> {
        let mut indexes = Vec::new();

        for i in 0..number_columns {
            if i % 4 == 0 {
                indexes.push(i);
            }
        }

        indexes
    }
}

pub struct Invaders {
    list: Vec<Invader>,
    timer: Timer,
    move_direction: i32,
    go_downwards: bool,
}

impl Invaders {
    pub fn new() -> Invaders {
        Invaders {
            list: Invader::create_level(2),
            timer: Timer::from_millis(1500),
            move_direction: 1,
            go_downwards: false,
        }
    }

    fn move_invaders(&mut self) {
        if self.move_direction > 0 {
            self.list
                .iter_mut()
                .for_each(|invader: &mut Invader| invader.x_index += 2);
        } else {
            self.list
                .iter_mut()
                .for_each(|invader: &mut Invader| invader.x_index -= 2);
        }

        self.move_direction *= -1;
    }

    fn increase_invaders_speed_by(&mut self, ms: u128) {
        let new_duration = max(self.timer.duration.as_millis() - ms, 250);
        self.timer = Timer::from_millis(new_duration as u64);
    }

    fn advance_invaders(&mut self) {
        self.list
            .iter_mut()
            .for_each(|invader| invader.y_index += 1);
    }

    pub fn try_kill_invaders_at(&mut self, x: usize, y: usize) -> bool {
        let mut enumerator = self.list.iter_mut().enumerate();
        if let Some((index, _)) = enumerator
                                            .find(|(_, i)| 
                                                i.x_index == x && i.y_index == y) {
            self.list.remove(index);
            
            return true; 
        }

        false
    }

    pub fn is_any_at_bottom(&self) -> bool {
        let max_invader_index = self.list.iter().map(|i| i.y_index).max().unwrap_or(0);
        if max_invader_index >= (NUMBER_ROWS - 1) {
            return true;
        }
        
        false
    }

    pub fn are_all_dead(&self) -> bool {
        self.list.iter().len() == 0
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        let time_left = self.timer.time_left.as_secs_f32();
        let duration = self.timer.duration.as_secs_f32();
        let invader_render = if time_left / duration > 0.5 { INVADERS_FORMS[0] } else { INVADERS_FORMS[1] };

        self.list
            .iter()
            .for_each(|invader| frame[invader.x_index][invader.y_index] = invader_render);
    }
}

impl Updatable for Invaders {
    fn update(&mut self, delta: Duration) {
        self.timer.update(delta);

        if !self.timer.ready {
            return;
        }

        self.timer.reset();
        
        if !self.go_downwards {
            self.move_invaders();
        } else {
            self.increase_invaders_speed_by(70);
            self.advance_invaders();
        }

        self.go_downwards = !self.go_downwards;
    }
}
