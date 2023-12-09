pub mod render;
pub mod frame;
pub mod player;
pub mod shot;
pub mod invaders;

pub const NUMBER_ROWS: usize = 20;
pub const NUMBER_COLUMNS: usize = 40;
pub const SHOT_FORM: &str = "|";
pub const PLAYER_FORM: &str = "A";
pub const INVADERS_FORMS: [&str; 2] = ["x", "+"];
