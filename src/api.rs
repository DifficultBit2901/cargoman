use std::time::Duration;

use colored::Colorize;
use crates_io_api::{Crate, CratesQuery, Sort, SyncClient};

const USER_AGENT: &'static str = "cargoman (https://github.com/DifficultBit2901/cargoman)";
const RATE_LIMIT_MS: u64 = 1500;

fn get_client() -> SyncClient {
    // Err is with invalid header, but that shouldn't change once built
    SyncClient::new(USER_AGENT, Duration::from_millis(RATE_LIMIT_MS)).unwrap()
}

pub fn search_pattern(pattern: &str, limit: u8, colored: bool) {
    let client = get_client();
    let query = CratesQuery::builder()
        .sort(Sort::Relevance)
        .search(pattern)
        .page_size(limit as u64)
        .build();
    let response = client.crates(query);
    match response {
        Err(e) => eprintln!("{}", e),
        Ok(page) => {
            for crt in page.crates {
                print_crate_short(crt, colored);
            }
        }
    }
}

fn print_crate_short(crt: Crate, colored: bool) {
    let name = crt.name;
    let keywords = crt.keywords;
    if colored {
        print!("{}", name.green());
        if let Some(keywords) = keywords {
            print!("({})", keywords.join(", ").blue());
        }
    } else {
        print!("{}", name);
        if let Some(keywords) = keywords {
            print!("({})", keywords.join(", "));
        }
    }
    println!();
}
