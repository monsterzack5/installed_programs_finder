pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            0 | 1 => Err("Not enough args! usage: <history file>"),
            2 => {
                let filename = args[1].clone();
                return Ok(Config { filename });
            }
            _ => Err("Too many args! usage: <history file>"),
        }
    }
}
