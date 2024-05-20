use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let code: String = fs::read_to_string(&args[1])?;
    let instructions: Vec<&str> = code.split("\n").collect();

    let mut line_num: usize = 0;
    let mut variables: HashMap<&str, &str> = HashMap::new();
    loop {
        if line_num >= instructions.len() {
            break;
        }

        let line: Vec<&str> = instructions[line_num].split(" ").collect();
        if line[0] == "data" {
            variables.insert(line[1], line[2]);
            
        } else if line[0] == "exit" {
            process::exit(0);

        } else {
            println!("error: this instruction does not exist");
        }

        line_num += 1;
    }

    Ok(())
}