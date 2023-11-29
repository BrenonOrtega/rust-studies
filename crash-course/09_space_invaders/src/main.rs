use std::io::{stdout, Stdout, self};

use crossterm::{terminal::EnterAlternateScreen, ExecutableCommand, cursor::{Hide, self}};

fn main() {
    let mut audio = rusty_audio::Audio::new();
    vec![("win","win.wav"),
        ("startup","startup.wav"),
        ("move","move.wav"),
        ("explode","explode.wav"),
        ("pew","pew.wav"),
        ("lose","lose.wav"), 
        ].iter()
        .for_each(|track| audio.add(track.0, track.1));

    let mut stdout = io::stdout();
    match stdout.execute(EnterAlternateScreen) {
        Ok(_) => {},
        Err(e) => println!("{:?}", e)
    };
    stdout.execute(cursor::Hide).unwrap();

    audio.play("startup");

    audio.wait();

}
