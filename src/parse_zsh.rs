use crate::config::Config;
use regex::Regex;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn norm_data(data: &str) -> String {
    // This function will:
    // change mulitple spaces into just one
    // Strip the first 15 (epoch + tty) from each line
    // remove erroneous " ' \ / symbols
    /* I should probably do this with just one regex, but this is easier s*/
    let stripped = Regex::new(" {2,}")
        .unwrap()
        .replace_all(&data, " ")
        .into_owned();
    let stripped = Regex::new("\"")
        .unwrap()
        .replace_all(&stripped, "")
        .into_owned();
    let stripped = Regex::new("'")
        .unwrap()
        .replace_all(&stripped, "")
        .into_owned();
    let stripped = Regex::new("/")
        .unwrap()
        .replace_all(&stripped, "")
        .into_owned();
    let stripped = Regex::new("\\\\")
        .unwrap()
        .replace_all(&stripped, "")
        .into_owned();

    let mut normalized = String::from("");
    for line in stripped.lines() {
        if line.len() > 15 {
            normalized.push_str(&line[15..line.len()]);
            normalized.push_str("\n");
        }
    }
    return normalized;
}

fn check_element(element: &str) -> bool {
    // todo: handle installed debs that start with a ,
    return !element.starts_with("-")
        && !element.starts_with(".")
        && !element.contains(" ")
        && element != ""
        && element != "install"
        && element != "remove"
        && element != "purge";
}

fn split_elements(data: &str) -> Vec<&str> {
    // expected input: `this that the_other-thing`
    // expected output: Vector of all the elements
    let split: Vec<&str> = data.split_whitespace().collect();
    let mut segmented: Vec<&str> = Vec::new();
    for item in split.iter() {
        if item.contains("&&") {
            return segmented;
        }
        segmented.push(item);
    }
    return segmented;
}

fn handle_line(line: &str) -> Vec<&str> {
    let mut elements: Vec<&str> = Vec::new();
    // if the line contains spaces, split into elements
    if line.contains(" ") {
        for line in split_elements(line) {
            if check_element(line) {
                elements.push(line);
            }
        }
        return elements;
    }

    if check_element(line) {
        elements.push(line);
    }

    return elements;
}

fn search(contents: &String) -> HashSet<String> {
    let contents = contents.to_lowercase();
    let mut results: HashSet<String> = HashSet::new();
    // change mulitple spaces into just one
    let contents = norm_data(&contents);
    // search for `apt install` and `apt remove|purge`
    for line in contents.lines() {
        if line.len() < 16 {
            continue;
        }

        if !(line.contains("install") || line.contains("remove") || line.contains("purge")) {
            continue;
        }

        if line.starts_with("sudo apt install ") {
            for element in handle_line(line) {
                results.insert(String::from(element));
            }
        }
        if line.starts_with("sudo apt purge ") || line.starts_with("sudo apt remove ") {
            for element in handle_line(line) {
                results.remove(&String::from(element));
            }
        }
    }
    return results;
}

pub fn run(config: Config) -> Result<HashSet<String>, Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let found_programs = search(&contents);

    return Ok(found_programs);
}
