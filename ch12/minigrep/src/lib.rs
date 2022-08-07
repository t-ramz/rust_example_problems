// this file should handle all of the logic for the program
use std::fs;
use std::error::Error;


#[derive(Debug)]
pub struct Config {
    query: String,
    filename_option: Option<String>,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.is_empty() {
            return Err("\n\tThe program must be used as:\n\tminigrep <text pattern> [<file name>]\n");
        }
        Ok(
            Config {
                query: args[0].clone(),
                filename_option: match args.get(1) {
                    None => None,
                    Some(string) => Some(string.clone()),
                },
            }
        )
    }
}

                        // err result type is a trait object
                        // specifies return must implement Error trait
                        // but not specific type, hence `dyn`amic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let Some(filename) = config.filename_option {
        read_and_search_file(config.query.as_str(), filename.as_str())?
    } else {
        // read all files in dir
        for entry in fs::read_dir(".")? {
            let entry_file = entry?;
            if entry_file.file_type()?.is_file() {
                if let Some(filename) = entry_file.path().to_str() {
                    read_and_search_file(config.query.as_str(), filename)?;
                }
            }
        }
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matched_results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matched_results.push(line.trim());
        }
    }
    matched_results
}

fn read_and_search_file(query: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;     // returns Result<String>
                                                      // ? operator return config_return_error_too_few_args
    for line in search(query, contents.as_str()) {
        println!("{filename}\t{line}");
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_valid_args() -> Vec<String> {
        vec![String::from("among"), String::from("us")]
    }

    #[test]
    fn config_return_error_too_few_args() {
        // arrange
        let empty_vec = Vec::new();
        let passed_args: &[String] = &empty_vec;

        // act
        let config_err = Config::new(passed_args).expect_err("Invalid config did not error");

        // assert
        assert!(config_err.contains("<text pattern>"));
    }
    #[test]
    fn config_return_valid_with_filename() {
        //arrange
        let args: &[String] = &get_valid_args();

        // act
        let config = Config::new(args).expect("Configuration was invalid");

        // assert
        assert_eq!(config.query, args[0]);
        if let Some(filename) = config.filename_option {
            assert_eq!(filename, args[1]);
        } else {
            panic!("Valid configuration filename not detected");
        }
    }
    #[test]
    fn config_return_valid_without_filename() {
        // assert
        let mut args: &[String] = &get_valid_args();
        args = &args[0..1];

        // act
        let config = Config::new(args).expect("Configuration was invalid");

        // assert
        assert_eq!(config.query, args[0]);
        if let Some(filename) = config.filename_option {
            panic!("Cosmic ray detected, filename value retrieved {}", filename);
        }
    }

    // Section: Test-Driven Development
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query,contents)
        );
    }
}
