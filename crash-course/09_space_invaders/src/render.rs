use std::{io::{Stdout, Write}, time::Duration};

use crossterm::{QueueableCommand, 
    style::{SetBackgroundColor, Color},
    terminal::{Clear, ClearType}, cursor};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, actual_frame: &Frame, last_frame: &Frame, force: bool) {
    if force {
        make_terminal_blue(stdout);
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    
    for (x, col) in actual_frame.iter().enumerate() {
        for (y, actual_frame_value) in col.iter().enumerate() {
            let last_frame_value = last_frame[x][y];
            if *actual_frame_value != last_frame_value || force {
                stdout.queue(cursor::MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", actual_frame_value);
            }
        }
    }

    stdout.flush().unwrap();
}

fn make_terminal_blue(stdout: &mut Stdout) {
    stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
    stdout.queue(Clear(ClearType::All)).unwrap();
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

pub trait Updatable {
    fn update(&mut self, delta: &Duration);
}