use std::env;
use std::process;

// Bringing in our dependency.
// minigrep is the name of our app.
use minigrep::Config;

fn main() {
    // Collect the args into a vector of strings.
    let args: Vec<String> = env::args().collect();

    // Call the function to parse.
    // unwrap_or_else() returns Ok variant or runs this closure if error.
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);

        // This terminates program with status code passed in.
        process::exit(1);
    });

    // Return all the args.
    println!("===args: {:?}", args);
    
    // Returns the saved args.
    println!("===query: {}", config.query);
    
    // Print out the filename and contents.
    println!("===filename: {}", config.filename);

    // run() also returns Result so we need to handle that with if-let.
    // If we supply a file that doesn't exist, `cargo run test test.txt`
    // The error returning -> The system cannot find the file specified. (os error 2)
    // Use of minigrep app name to use the function stored in lib.rs.
    if let Err(e) = minigrep::run(config) {
        eprintln!("===Application error: {}", e);
        process::exit(1);
    }
}
