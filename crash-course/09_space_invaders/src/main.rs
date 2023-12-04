use std::{io::{Stdout, self, stdout}, fmt::Error, sync::mpsc::{self, Sender}, thread::{self, JoinHandle}, time::Duration};
use crossterm::{
    ExecutableCommand, 
    cursor,
    terminal::{EnterAlternateScreen, self, LeaveAlternateScreen},
    event::{self, Event, KeyCode}};

use rusty_audio::Audio;
use space_invaders::{frame::{Frame, new_frame}, render::{self, render, Drawable}, player::Player};

fn main() -> Result<(), Error> {
    let mut audio = setup_audio();
    let stdout = setup_game_screen();
    thread::sleep(Duration::from_millis(7_000));
    audio.play("startup");

    game_loop(&mut audio);

    close_game_cleanup(audio, stdout);

    Ok(())
}

fn setup_audio() -> rusty_audio::prelude::Audio {
    let mut audio = rusty_audio::Audio::new();
    vec![("win","win.wav"),
        ("startup","startup.wav"),
        ("move","move.wav"),
        ("explode","explode.wav"),
        ("pew","pew.wav"),
        ("lose","lose.wav"), 
        ].iter()
        .for_each(|track| audio.add(track.0, track.1));
    audio
}

fn setup_game_screen() -> Stdout {
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen).unwrap();
    stdout.execute(cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();

    stdout
}


fn game_loop(audio: &mut Audio) {
    let (thread, tx) = get_frame_rendering_sender();
    let mut player = Player::new();

    'gameloop: loop {
        while event::poll(Duration::default()).unwrap() {
            let event: Event = event::read().unwrap();
            
            match event {
                Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') => {
                            audio.play("lose");
                            break 'gameloop;
                        },

                        KeyCode::Left => player.move_left(),

                        KeyCode::Right => player.move_right(),
                        
                        KeyCode::Char(' ') | KeyCode::Up => player.shoot(),

                        _ => {}
                    }
                },
                _ => {}
            } 

            let mut frame = new_frame();
            player.draw(&mut frame);

            let _ = tx.send(frame);
            thread::sleep(Duration::from_millis(6));
        }
    }  

    drop(tx);
    thread.join().unwrap();
}


fn get_frame_rendering_sender() -> (JoinHandle<()>, Sender<Frame>) {
    let (handle_tx, handle_rx) = mpsc::channel::<Frame>();
    let thread = thread::spawn(move || {
        let mut last_frame: Frame = new_frame();
        let stdout = &mut stdout();
        render::render(stdout, &last_frame,  &last_frame, true);
        
        loop {
            match handle_rx.recv() {
                Ok(frame) => {
                    let curr_frame: Frame = frame;
                    render(stdout, &curr_frame, &last_frame, false);
                    last_frame = curr_frame;
                },
                Err(e) => {
                    break;
                },
            };
        }
    });

    (thread, handle_tx)
}

fn close_game_cleanup(audio: Audio, mut stdout: Stdout) {
    audio.wait();
    stdout.execute(LeaveAlternateScreen).unwrap();
    stdout.execute(cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
    drop(stdout);
    drop(audio);
}
