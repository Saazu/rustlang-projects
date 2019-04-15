use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

//Command line arguments struct
//query: term to be searched
//filename: file that will be searched
pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

//this function processes command line arguments
//Returns struct that holds search term(query) and file to be searched(filename)
impl Config {
	pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string"),
		};

		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a file name"),
		};

		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config { query, filename, case_sensitive })
	}
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
	let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    //set case sensitive variant based on environmental variables
    let results = if config.case_sensitive {
    	search(&config.query, &contents)
    } else {
    	search_case_insensitive(&config.query, &contents)
    };

    for line in results {
    	println!("{}", line);
    }

    Ok(())
}

/*       Search function without iterators
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}
	results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}
	results
}
*/

//////////Search functions refactored with iterators
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines()
		.filter(|line| line.contains(query))
		.collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	contents.lines()
		.filter(|line| line.to_lowercase().contains(&query))
		.collect()
}
 ///////////////////////// TESTS BEGIN HERE ///////////////////////////////////
//////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
		assert_eq!(
			vec!["safe, fast, productive."], search(query, contents)
		);
	}
}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
		assert_eq!(
			vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
	}

