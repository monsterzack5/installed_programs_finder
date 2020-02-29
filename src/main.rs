use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process;

mod check_installed;
mod config;
mod parse_zsh;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = config::Config::new(&args).unwrap_or_else(|e| {
        println!("Config Error: {}", e);
        process::exit(1);
    });

    let found_programs = match parse_zsh::run(config) {
        Ok(results) => results,
        Err(e) => panic!("Error parsing zsh file: {}", e),
    };

    let def_installed_programs = match check_installed::run(&found_programs) {
        Ok(results) => results,
        Err(e) => panic!("Error checking installed system packages: {}", e),
    };

    let results = def_installed_programs.intersection(&found_programs); // .collect::<HashSet<String>>();
    let mut file = match OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("installed-programs.log")
    {
        Ok(file) => file,
        Err(e) => panic!("{}", e),
    };
    writeln!(file, "These are the programs you have installed with apt:").unwrap();
    for program in results.into_iter() {
        writeln!(file, "{}", program).expect("Error writing installed programs to a file");
    }
}
