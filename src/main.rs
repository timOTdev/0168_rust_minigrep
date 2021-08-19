use std::env;

fn main() {
    // Collect the args into a vector of strings.
    let args: Vec<String> = env::args().collect();

    // Print out the args.
    print!("{:?}", args);
}
