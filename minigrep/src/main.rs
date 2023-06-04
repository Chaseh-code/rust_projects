use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    //let query = &args[1];
    //let file_path = &args[2];
    //let (query, file_path) = parse_config(&args);
    //let config = parse_config(&args);
    //let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {}", query);
    //println!("In file {}", file_path);
    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);
    //dbg!(args);

    //let contents = fs::read_to_string(file_path)
    //let contents = fs::read_to_string(config.file_path)
    //    .expect("Should have been able to read the file");

    //println!("With text:\n{contents}");
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
}