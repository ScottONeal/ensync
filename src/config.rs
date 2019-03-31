pub struct Config {
    pub source_directory: String,
    pub destination:      String
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 3 {
            return Err("You must supply a source directory and a destination");
        }

        let source_directory = args[1].clone();
        let destination      = args[2].clone();

        return Ok(Config { source_directory, destination });
    }
}