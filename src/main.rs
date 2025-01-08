use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let data = fs::read_to_string(&args[1]).expect("Unable to read file");

    let buffer = data.split("\n").collect::<Vec<&str>>();

    for line in buffer {
        let instructions = line.split(" ").collect::<Vec<&str>>();
        for instruction in instructions {
            match instruction {
                "BIPUSH" => println!("0x10"),
                "IADD" => println!("0x60"),
                _ => { println!("Unknown instruction: {}", instruction); },
            }
        }
    }
}