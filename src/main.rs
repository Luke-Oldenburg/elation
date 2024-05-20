use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let code: String = fs::read_to_string(&args[1])?;
    let instructions: Vec<&str> = code.split("\n").collect();

    let mut line_num: usize = 0;
    loop {
        if line_num >= instructions.len() {
            break;
        }

        let line: Vec<&str> = instructions[line_num].split(" ").collect();
        match line[0] {
            "exit" => process::exit(0),
            _ => println!("error: this instruction does not exist")
        }

        line_num += 1;
    }

    Ok(())
}
