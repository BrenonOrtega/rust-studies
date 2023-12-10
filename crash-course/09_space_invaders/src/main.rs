use rusty_audio::Audio;

use std::{io::{Stdout, self, stdout},
    fmt::Error, sync::mpsc::{self, Sender},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
    ops::ControlFlow};

use crossterm::{
    ExecutableCommand, 
    cursor,
    terminal::{EnterAlternateScreen, self, LeaveAlternateScreen},
    event::{self, Event, KeyCode}};

use space_invaders::{
    frame::{Frame, new_frame, Drawable},
    render::{self, render, Updatable},
    player::Player,
    invaders::Invaders, NUMBER_ROWS, INVADERS_FORMS};

fn main() -> Result<(), Error> {
    let mut audio = setup_audio();
    let stdout = setup_game_screen();
    
    thread::sleep(Duration::from_millis(4_000));
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
        .for_each(|track: &(&str, &str)| audio.add(track.0, track.1));
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
    let mut invaders = Invaders::new();
    let mut instant = Instant::now();

    'gameloop: loop {
        let delta = instant.elapsed();
        
        while event::poll(Duration::default()).unwrap() {
            let event: Event = event::read().unwrap();
            
            if let ControlFlow::Break(_) = handle_key_press(event, audio, &mut player) {
                break 'gameloop;
            }
        }
        
        instant = Instant::now();
        
        let mut frame: Frame = new_frame();

        update_all(vec![&mut player, &mut invaders], delta);
        draw_all(vec![&player, &invaders], &mut frame);

        let _ = tx.send(frame);

        if player.has_killed_any_invader(&mut invaders) {
            audio.play("explode");
        }

        if invaders.are_all_dead() { 
            audio.play("win");
            thread::sleep(Duration::from_millis(500));
            break 'gameloop;
        }

        if invaders.is_any_at_bottom() {
            audio.play("lose");
            thread::sleep(Duration::from_millis(500));
            break 'gameloop;
        }

        thread::sleep(Duration::from_millis(20));
    }  

    drop(tx);
    thread.join().unwrap();
}

fn draw_all(drawables: Vec<&dyn Drawable>, frame: &mut Frame) {
    drawables.iter().for_each(|a| a.draw(frame));
}

fn update_all(mut updatables: Vec<&mut dyn Updatable>, delta: Duration) {
    updatables.iter_mut()
        .for_each(|updatable| updatable.update(delta));
}

fn handle_key_press(event: Event, audio: &mut Audio, player: &mut Player) -> ControlFlow<()> {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    audio.play("lose");
                    return ControlFlow::Break(());
                },

                KeyCode::Left => player.move_left(),

                KeyCode::Right => player.move_right(),
            
                KeyCode::Char(' ') | KeyCode::Up => { 
                    if player.shoot() {
                        audio.play("pew");
                   };
                },

                _ => {}
            }
        }

    ControlFlow::Continue(())
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
                Err(_e) => {
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
