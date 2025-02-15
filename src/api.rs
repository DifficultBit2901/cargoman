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

pub fn crate_info(crt: &str, colored: bool) {
    let client = get_client();
    let response = client.get_crate(crt);
    match response {
        Err(e) => eprintln!("{}", e),
        Ok(crate_response) => {
            let crate_data = crate_response.crate_data;
            let categories = crate_response.categories;
            let keywords = crate_response.keywords;
            let versions = crate_response.versions;
            if colored {
                println!("Crate-Name: {}", crate_data.name.green());
                let latest_version = versions[0].clone();
                println!(
                    "License(s): {}",
                    latest_version
                        .license
                        .map_or("None".red(), |license| license.blue())
                );
                if let Some(doc_url) = crate_data.documentation {
                    println!("Documentation: {}", doc_url.blue());
                }
                if let Some(homepage) = crate_data.homepage {
                    println!("Homepage: {}", homepage.blue());
                }
                if let Some(repo) = crate_data.repository {
                    println!("Repository: {}", repo.blue());
                }
                println!("Downloads: {}", crate_data.downloads.to_string().yellow());

                let version_names: Vec<String> =
                    versions.iter().map(|version| version.num.clone()).collect();
                print!("Versions: [");
                for (idx, item) in version_names.iter().enumerate() {
                    print!("\"{}\"", item.yellow());
                    if idx + 1 < version_names.len() {
                        print!(", ");
                    }
                }
                println!("]");

                let keywords: Vec<String> = keywords
                    .iter()
                    .map(|keyword| keyword.keyword.clone())
                    .collect();
                print!("Keywords: [");
                for (idx, item) in keywords.iter().enumerate() {
                    print!("\"{}\"", item.blue());
                    if idx + 1 < keywords.len() {
                        print!(", ");
                    }
                }
                println!("]");

                let categories: Vec<String> =
                    categories.iter().map(|cat| cat.category.clone()).collect();
                print!("Categories: [");
                for (idx, item) in categories.iter().enumerate() {
                    print!("\"{}\"", item.blue());
                    if idx + 1 < categories.len() {
                        print!(", ");
                    }
                }
                println!("]");

                let features = &latest_version.features;
                let feature_names: Vec<String> = features.clone().into_keys().collect();
                print!("Features: [");
                for (idx, feature) in feature_names.iter().enumerate() {
                    if feature == "default" {
                        continue;
                    }
                    print!("\"{}\"", feature.blue());
                    if idx + 1 < feature_names.len() {
                        print!(", ");
                    }
                }
                println!("]");

                let default_features = match features.get("default") {
                    Some(features) => features.to_owned(),
                    None => Vec::new(),
                };
                print!("Default Features: [");
                for (idx, item) in default_features.iter().enumerate() {
                    print!("\"{}\"", item.blue());
                    if idx + 1 < default_features.len() {
                        print!(", ");
                    }
                }
                println!("]");

                if let Some(description) = crate_data.description {
                    println!("{}", description);
                }
            } else {
                println!("Crate-Name: {}", crate_data.name);
                let latest_version = versions[0].clone();
                println!(
                    "License(s): {}",
                    latest_version.license.unwrap_or("None".to_owned())
                );
                if let Some(doc_url) = crate_data.documentation {
                    println!("Documentation: {}", doc_url);
                }
                if let Some(homepage) = crate_data.homepage {
                    println!("Homepage: {}", homepage);
                }
                if let Some(repo) = crate_data.repository {
                    println!("Repository: {}", repo);
                }
                println!("Downloads: {}", crate_data.downloads.to_string());

                let version_names: Vec<String> =
                    versions.iter().map(|version| version.num.clone()).collect();
                print!("Versions: [");
                for (idx, item) in version_names.iter().enumerate() {
                    print!("\"{}\"", item);
                    if idx + 1 < version_names.len() {
                        print!(", ");
                    }
                }
                println!("]");

                let keywords: Vec<String> = keywords
                    .iter()
                    .map(|keyword| keyword.keyword.clone())
                    .collect();
                print!("Keywords: [");
                for (idx, item) in keywords.iter().enumerate() {
                    print!("\"{}\"", item);
                    if idx + 1 < keywords.len() {
                        print!(", ");
                    }
                }
                println!("]");

                let categories: Vec<String> =
                    categories.iter().map(|cat| cat.category.clone()).collect();
                print!("Categories: [");
                for (idx, item) in categories.iter().enumerate() {
                    print!("\"{}\"", item);
                    if idx + 1 < categories.len() {
                        print!(", ");
                    }
                }
                println!("]");

                let features = &latest_version.features;
                let feature_names: Vec<String> = features.clone().into_keys().collect();
                print!("Features: [");
                for (idx, feature) in feature_names.iter().enumerate() {
                    if feature == "default" {
                        continue;
                    }
                    print!("\"{}\"", feature);
                    if idx + 1 < feature_names.len() {
                        print!(", ");
                    }
                }
                println!("]");

                let default_features = match features.get("default") {
                    Some(features) => features.to_owned(),
                    None => Vec::new(),
                };
                print!("Default Features: [");
                for (idx, item) in default_features.iter().enumerate() {
                    print!("\"{}\"", item);
                    if idx + 1 < default_features.len() {
                        print!(", ");
                    }
                }
                println!("]");

                if let Some(description) = crate_data.description {
                    println!("{}", description);
                }
            }
        }
    }
}

fn print_crate_short(crt: Crate, colored: bool) {
    let name = crt.name;
    if colored {
        print!("{}", name.green());
    } else {
        print!("{}", name);
    }
    println!();
}
