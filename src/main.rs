mod counter;
use counter::recursive_count_tokens;
use std::env::args;

fn main() {
    // parse commandline args
    let args: Vec<String> = args().collect();
    let folder_path = &args[1];
    let result = recursive_count_tokens(folder_path);
    match result {
        Ok(count) => println!("Total tokens: {}", count),
        Err(e) => eprintln!("Error: {}", e),
    }
}
