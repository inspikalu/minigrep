use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //let args: Vec<_>= env::args().collect();
    //Vec<_> this tells the compiler to infer the type


    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {} \n\n\n\n\n\n\n\n", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}

