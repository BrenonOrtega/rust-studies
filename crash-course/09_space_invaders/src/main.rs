use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = setup_game_audio();
    
    Ok(())
}

fn setup_game_audio() -> Audio {
    let mut audio = Audio::new();

    vec![("explode", "explode.wav"),
    ("lose", "lose.wav"),
    ("move", "move.wav"),
    ("pew", "pew.wav"),
    ("startup", "startup.wav"),
    ("win", "win.wav"),
    ].iter()
    .for_each(|value| audio.add(value.0, value.1));

    audio
}
