use std::env;

fn main() {
    // Collect the args into a vector of strings.
    let args: Vec<String> = env::args().collect();

    // Pull out individual args.
    let query = &args[1];
    let filename = &args[2];

    // Return all the args.
    println!("{:?}", args);

    // Returns the saved args.
    println!("Searching for {}", query);
    println!("In file {}", filename);

    // ==Output
    // Compiling minigrep v0.1.0 (C:\timh1203\coding\rust_minigrep)
    //     Finished dev [unoptimized + debuginfo] target(s) in 1.01s
    //     Running `target\debug\minigrep.exe test sample.txt`
    // Searching for test
    // In file sample.txt
    // PS C:\timh1203\coding\rust_minigrep> cargo run test sample.txt
    // Compiling minigrep v0.1.0 (C:\timh1203\coding\rust_minigrep)
    //     Finished dev [unoptimized + debuginfo] target(s) in 0.73s
    //     Running `target\debug\minigrep.exe test sample.txt`
    // ["target\\debug\\minigrep.exe", "test", "sample.txt"]
    // Searching for test
    // In file sample.txt
}
