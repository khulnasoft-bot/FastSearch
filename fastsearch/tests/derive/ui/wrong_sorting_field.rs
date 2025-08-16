use fastsearch::Fastsearch;
use serde::{Serialize, Deserialize};

#[derive(Fastsearch, Serialize, Deserialize)]
#[fastsearch(default_sorting_field = "wrong_field")]
struct Company {
    company_name: String,
    num_employees: i32,
    country: String,
}

fn main() {}
