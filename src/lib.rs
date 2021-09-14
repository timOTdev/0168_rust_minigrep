use std::fs;
use std::error::Error;

// Private by default, need to add pub to fn, struct, and fields.

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    // This opens the file and returns Result type.
    // Use expect() to handle Ok and Err.
    // ? handles Result error type and returns it from the function.
    let contents = fs::read_to_string(config.filename)?;

    // // Prints out the contents of the file.
    // println!("===contents:\n{}", contents);

    // We want to print out the whole line.
    for line in search(&config.query, &contents) {
      println!("{}", line);
    }

    // Returns unit type if it reaches here.
    Ok(())

    // ==Output running `cargo run the poem.txt`
    // ===
    // ["target\\debug\\minigrep.exe", "the", "poem.txt"]
    // ===
    // Searching for the
    // ===
    // In file poem.txt
    // With text:
    // I'm nobody! Who are you?
    // Are you nobody, too?
    // Then there's a pair of us - don't tell!
    // They'd banish us, you know.

    // How dreary to be somebody!
    // How public, like a frog
    // To tell your name the livelong day
    // To an admiring bog!
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // Changed the fn name to new because it's a convention for naming constructors.
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough parameters.");
        }

        // Pull out individual args.
        // Index 0 is the binary so we skip that.
        // We clone the strings here to not take ownership.
        // Not performant but easiest to do right now.
        let query = args[1].clone();
        let filename = args[2].clone();

        // Make a new config to relate the data.
        // We returned a tuple before but that doesn't show relationship.
        Ok(Config{query, filename})
    }
}

// We need to implement lifetime because we are returning
// a reference from the function.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  // Add empty collection.
  let mut results = Vec::new();

  // Loop through each line of the contents.
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  // Return vector.
  results
}

pub fn search_case_insensitive<'a>(
  _query: &str, _contents: &'a str
) -> Vec<&'a str> {
  vec![]
}

#[cfg(test)]
mod tests {
  use super::*;

  // Updated this test code to test for case insensitive.
  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    // Uses search function and expect lines that contains our query.
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUst";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    // Testing that should return 2 lines in contents.
    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    )
  }
}
