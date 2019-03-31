use std::path::Path;
use std::fs;
use walkdir::WalkDir;
mod config;

fn main() {
    let mut options;

    match config::Config::new() {
        Ok(o) => options = o,
        Err(e)   => {
            println!("{}", e);
            std::process::exit(1);
        }
    }

    setup_destination(&options.destination);

    println!("source directory: {}\ndestination: {}", options.source_directory, options.destination);
    for entry in WalkDir::new(options.source_directory).into_iter().filter_map(|e| e.ok()) {
        println!("{}", entry.path().display());
    }
}

fn setup_destination(destination: &String) {
    if Path::new(destination).exists() == false {
        match fs::create_dir_all(destination) {
            Ok(_)    => {},
            Err(err) => println!("! {:?}", err.kind())
        }
    }
}