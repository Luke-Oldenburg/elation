use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let code: String = fs::read_to_string(&args[1])?;
    let instructions: Vec<&str> = code.lines().collect();

    let stdin = io::stdin();
    let mut line_num: usize = 0;
    let mut variables: HashMap<&str, String> = HashMap::new();
    let mut labels: HashMap<&str, String> = HashMap::new();
    loop {
        if line_num >= instructions.len() {
            break;
        }

        let line: Vec<&str> = instructions[line_num].split_whitespace().collect();
        if line[0] == "calculate" {
            let argument1: i32 = variables.get(line[1]).expect("REASON").parse().unwrap();
            let argument2: i32 = variables.get(line[3]).expect("REASON").parse().unwrap();
            if line[2] == "+" {
                variables.insert(line[4], (argument1 + argument2).to_string());

            } else if line[2] == "-" {
                variables.insert(line[4], (argument1 - argument2).to_string());

            } else if line[2] == "*" {
                variables.insert(line[4], (argument1 * argument2).to_string());

            } else if line[2] == "/" {
                variables.insert(line[4], (argument1 / argument2).to_string());

            } else if line[2] == "%" {
                variables.insert(line[4], (argument1 % argument2).to_string());

            } else if line[2] == "^" {
                variables.insert(line[4], i32::pow(argument1, argument2.try_into().unwrap()).to_string());
            }

        } else if line[0] == "compare" {
            let argument1: i32 = variables.get(line[1]).expect("REASON").parse().unwrap();
            let argument2: i32 = variables.get(line[3]).expect("REASON").parse().unwrap();
            if line[2] == "=" {
                variables.insert(line[4], (argument1 == argument2).to_string());

            } else if line[2] == "!=" {
                variables.insert(line[4], (argument1 != argument2).to_string());

            } else if line[2] == ">" {
                variables.insert(line[4], (argument1 > argument2).to_string());

            } else if line[2] == "<" {
                variables.insert(line[4], (argument1 < argument2).to_string());

            } else if line[2] == ">=" {
                variables.insert(line[4], (argument1 >= argument2).to_string());

            } else if line[2] == "<=" {
                variables.insert(line[4], (argument1 <= argument2).to_string());
            }

        } else if line[0] == "concat" {
            variables.insert(line[3], variables.get(line[1]).unwrap().to_owned() + variables.get(line[2]).unwrap());

        } else if line[0] == "data" {
            variables.insert(line[1], line[2].to_string());

        } else if line[0] == "exit" {
            process::exit(0);

        } else if line[0] == "jump" {
            line_num = labels.get(line[1]).expect("REASON").parse().unwrap();

        } else if line[0] == "jump_if" {
            if variables.get(line[1]).expect("REASON").parse().unwrap() {
                line_num = labels.get(line[2]).expect("REASON").parse().unwrap();
            }

        } else if line[0] == "jump_if_else" {
            if variables.get(line[1]).expect("REASON").parse().unwrap() {
                line_num = labels.get(line[2]).expect("REASON").parse().unwrap();

            } else {
                line_num = labels.get(line[3]).expect("REASON").parse().unwrap();
            }

        } else if line[0] == "label" {
            labels.insert(line[1], line_num.to_string());

        } else if line[0] == "print" {
            println!("{}", variables.get(line[1]).unwrap());

        } else if line[0] == "read" {
            let mut user_input = String::new();
            stdin.read_line(&mut user_input)?;
            variables.insert(line[1], user_input.split_whitespace().collect::<Vec<&str>>()[0].to_string());

        } else {
            println!("error: this instruction does not exist");
        }

        line_num += 1;
    }

    Ok(())
}