use fastsearch::Fastsearch;
use serde::{Serialize, Deserialize};

#[derive(Fastsearch, Serialize, Deserialize)]
enum SomeEnum {
    First,
    Second,
    Third,
    Other(String),
}

fn main() {}
