use crate::change_log::ChangeLogTrait;
use crate::change_log::ChangeLog;
use crate::configurations::Configuration;

use dotenv::dotenv;
use serde::de::value;

mod change_log;
mod configurations;

fn main() {
    let _ = dotenv();
    let configuration: Configuration = match envy::from_env::<Configuration>() {
        Ok(value) => {value},
        Err(value) => { panic!("{}", value)},
    };

    println!("{}", serde::Serialize(configuration));

    let token = std::env::var("MY_API_TOKEN").expect("MY_API_TOKEN must be set");

    println!("{}", token);
    let change_log = ChangeLog::new(
        "DeleteAircraft",
        "SPRKS", "SPRKA", "costaob", "TailAllocation");

    let insert = change_log.to_sql("change_logs");
    
    println!("{}", insert);
}

fn from_str(value: &str) -> String {
    String::from(value)
}