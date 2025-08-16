use fastsearch::Fastsearch;
use serde::{Serialize, Deserialize};
#[derive(Fastsearch, Serialize, Deserialize)]
struct Company {
    company_name: String,
    num_employees: i32,
    #[fastsearch(facet)]
    #[fastsearch(facet)]
    country_code: String,
}

fn main() {}
