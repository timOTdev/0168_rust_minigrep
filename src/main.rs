use std::env;
use std::fs;

fn main() {
    // Collect the args into a vector of strings.
    let args: Vec<String> = env::args().collect();

    // Pull out individual args.
    // Index 0 is the binary so we skip that.
    let query = &args[1];
    let filename = &args[2];

    // ==Return all the args.
    println!("===args: {:?}", args);
    
    // ==Returns the saved args.
    println!("===query: {}", query);
    
    // Print out the filename and contents.
    println!("===filename: {}", filename);

    // This opens the file and returns Result type.
    // Use expect() to handle Ok and Err.
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    // Prints out the contents of the file.
    println!("===contents:\n{}", contents);

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
