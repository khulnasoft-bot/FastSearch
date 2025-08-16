use fastsearch::Fastsearch;
use serde::{Serialize, Deserialize};

#[derive(Fastsearch, Serialize, Deserialize)]
#[fastsearch(default_sorting_field = company_name)]
struct Company {
    company_name: String,
    num_employees: i32,
    #[fastsearch(facet)]
    country: String,
}

fn main() {}
