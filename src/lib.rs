use std::{fs, error::Error, env};

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {	
	pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {	
		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string!"),
		};	

		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a input filename!"),
		};

		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config{ query, filename, case_sensitive })
	}
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;
	
	let result = if config.case_sensitive {
		case_sensitive_search(&config.query, &contents)
	}
	else {
		case_insensitive_search(&config.query, &contents)
	};

	for line in result {
		println!("{}", line);
	}

	Ok(())
}


pub fn case_sensitive_search<'a>(query: &str, contents:&'a str) -> Vec<&'a str> {
	contents.lines()
		.filter(|line| line.contains(query))
		.collect()
}


pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut result: Vec<&str> = Vec::new();
	for line in contents.lines() {
		if line.to_lowercase().contains(&query.to_lowercase()) {
			result.push(line);
		}
	}

	result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn passing_single_argument() {
     		let test_args = vec!["filepath".to_string(), "e1".to_string()];
     		match Config::new(&test_args) {
     			Ok(_) => (),
     			Err(e) => panic!("Failed with: {}", e),
     		}
    }

    #[test]
    fn passing_two_arguments() {
    	let test_args = vec!["filepath".to_string(), "e1".to_string(), "e2".to_string()];
    	match Config::new(&test_args) {
    		Ok(_) => (),
    		Err(e) => panic!("Failed with: {}", e),
    	}
    }

    #[test]
    fn case_sensitive() {
    	let query = "duct";
    	let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(vec!["safe, fast, productive."],
					case_sensitive_search(query, contents)
		);
    }

    #[test]
    fn case_insensitive() {
    	let query = "rUst";
    	let contents = "\
Rust:
safe, fast, productive.
Trust me.";
		
		assert_eq!(vec!["Rust:", "Trust me."],
					case_insensitive_search(query, contents)
		);
    }
}
