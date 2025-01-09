use std::{env, fs};

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let data = fs::read_to_string(&args[1]).expect("Unable to read file");

    let buffer = data.split("\n").collect::<Vec<&str>>();

    let mut file_contents_base64: Vec<u8> = Vec::new();
    file_contents_base64.push(0x00u8);

    for line in buffer {
        if line.starts_with("#") || line.starts_with("//") {
            continue;
        }
        let instructions = line.split(" ").collect::<Vec<&str>>();
        for instruction in instructions {
            match instruction {
                "BIPUSH" => file_contents_base64.push(0x10),
                "DUP" => file_contents_base64.push(0x59),
                "GOTO" => file_contents_base64.push(0xA7),
                "IADD" => file_contents_base64.push(0x60),
                "IAND" => file_contents_base64.push(0x7E),
                "IFEQ" => file_contents_base64.push(0x99),
                "IFLT" => file_contents_base64.push(0x9B),
                "IF_ICMPEQ" => file_contents_base64.push(0x9F),
                "IINC" => file_contents_base64.push(0x84),
                "ILOAD" => file_contents_base64.push(0x15),
                "INVOKEVIRTUAL" => file_contents_base64.push(0xB6),
                "IOR" => file_contents_base64.push(0x80),
                "IRETURN" => file_contents_base64.push(0xAC),
                "ISTORE" => file_contents_base64.push(0x36),
                "ISUB" => file_contents_base64.push(0x64),
                "LDC_W" => file_contents_base64.push(0x13),
                "NOP" => file_contents_base64.push(0x00),
                "POP" => file_contents_base64.push(0x57),
                "SWAP" => file_contents_base64.push(0x5F),
                "//" => return,
                "#" => return,
                "" => continue,
                _ => {
                    let signed_value = u8::from_str_radix(instruction, 16)
                        .unwrap_or_else(|_| panic!("Invalid hex value: {}", instruction));


                    // Konvertieren in u8, um es in den Vec zu pushen
                    let unsigned_value = signed_value as u8;

                    file_contents_base64.push(unsigned_value);

                },
            }
        }
    }

    let path = &args[1].replace("jbyte", "ijvm");
    fs::write(path, file_contents_base64).unwrap();
}