use std::error::Error;

pub mod config;

mod search;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search::search_case_sensitive(&config.query, &contents)
    } else {
        search::search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
