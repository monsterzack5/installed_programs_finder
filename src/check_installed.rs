use std::collections::HashSet;
use std::error::Error;
use std::process::Command;

fn ask_apt(element: String) -> bool {
    let output = Command::new("apt")
        .arg("-qq")
        .arg("list")
        .arg(element)
        .output()
        .unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    return output.contains("[installed]");
}

pub fn run(elements: &HashSet<String>) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut found_programs: HashSet<String> = HashSet::new();
    for element in elements.iter() {
        if ask_apt(element.clone()) {
            found_programs.insert(element.clone());
        }
    }
    return Ok(found_programs);
}
