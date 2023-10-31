
use std::collections;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Configuration {
    ip: String,
    port: u16,
    connection_strings: collections::HashMap<&'static str, String>,
}

fn default_ip() -> &'static str { "localhost" }
