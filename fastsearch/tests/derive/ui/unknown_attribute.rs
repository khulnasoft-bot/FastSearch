use fastsearch::Fastsearch;
use serde::{Serialize, Deserialize};

#[derive(Fastsearch, Serialize, Deserialize)]
struct Company {
    company_name: String,
    num_employees: i32,
    #[fastsearch(facets)]
    country: String,
    keywords: Option<Vec<String>>,
}

fn main() {}
