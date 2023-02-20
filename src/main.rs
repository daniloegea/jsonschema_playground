use std::env;
use std::fs;

mod validate;

// WARNING: It doesn't support all the propoerties support by Netplan.

fn main() {
    let schema = validate::schema::build_schema().unwrap();

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 0 {
        println!("Try passing a bunch of netplan yamls as parameters");
        return;
    }

    for file in &args {
        println!("Parsing {file}");
        let yaml = fs::read_to_string(file).expect("Failed to open file");

        let result = validate::validate(&schema, &yaml);
        match result {
            Err(error) => {
                println!("Validation failed for file {file}");
                println!("Error: {error}");
            }
            Ok(_) => {
                println!("File {file} is valid");
            }
        }
    }
}
