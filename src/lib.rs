use std::fs;
use std::error::Error;
use std::env;

// Private by default, need to add pub to fn, struct, and fields.

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    // This opens the file and returns Result type.
    // Use expect() to handle Ok and Err.
    // ? handles Result error type and returns it from the function.
    let contents = fs::read_to_string(config.filename)?;

    // // Prints out the contents of the file.
    // println!("===contents:\n{}", contents);

    // // We want to print out the whole line.
    // for line in search(&config.query, &contents) {
    //   println!("{}", line);
    // }

    // Now we want to use our case_sensitive field.
    // Run appropriate fn based on boolean.
    let results = if config.case_sensitive {
      search(&config.query, &contents)
    } else {
      search_case_insensitive(&config.query, &contents)
    };
    
    // We want to print out each line from results.
    for line in results {
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
    pub case_sensitive: bool,
}

impl Config {
    // Changed the fn name to new because it's a convention for naming constructors.
    // Now we changed it to take in Arg struct which is an Iterator.
    // We need to make args mutable because we are iterating over it.
    // We also need it to be a static string literal reference because it needs to
    // last the whole program.
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // The first iteration, we are discarding because it's just the path to the file.
        args.next();

        // The next index will be our query from the command line.
        // next() returns Result so we need to handle it.
        // Note that we are taking ownership of the string.
        let query = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a query string"),
        };
        
        // Same thing with filename.
        // Note that we are taking ownership of the string.
        let filename = match args.next() {
          Some(arg) => arg,
          None => return Err("Didn't get a query string"),
        };

        // // Removed because we used Iterator instead.
        // // Pull out individual args.
        // // Index 0 is the binary so we skip that.
        // // We clone the strings here to not take ownership.
        // // Not performant but easiest to do right now.
        // let query = args[1].clone();
        // let filename = args[2].clone();
        
        // Need to add environmental variable.
        // env::var returns Result item to check if exists.
        // is_err checks if ok, and returns boolean.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // Make a new config to relate the data.
        // We returned a tuple before but that doesn't show relationship.
        // Now, Config will take ownership of the 2 Iterable derived values.
        Ok(Config{query, filename, case_sensitive})
    }
}

// We need to implement lifetime because we are returning
// a reference from the function.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  // We use iterators instead since it has built in methods.
  // collect() knows the output type because we specified it on the output lifetime.
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()

  // // Add empty collection.
  // let mut results = Vec::new();

  // // Loop through each line of the contents.
  // for line in contents.lines() {
  //   if line.contains(query) {
  //     results.push(line);
  //   }
  // }

  // // Return vector.
  // results
}

pub fn search_case_insensitive<'a>(
  query: &str, contents: &'a str
) -> Vec<&'a str> {
  // We convert our query to all lower case.
  let query = query.to_lowercase();
  let mut results = Vec::new();
  
  for line in contents.lines() {
    // We convert our line to all lower case.
    // to_lowercase() method returns a new string.
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
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
