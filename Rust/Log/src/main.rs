use std::fs;

fn validate_email(input: String) -> Result<(), String> {
    if input.contains("@") {
        Ok(())
    } else {
        Result::Err("Its not a valid email".to_string())
    }
}
fn main() {
    match fs::read_to_string("log.txt") {
        Result::Ok(value) => {
            println!("{:#?}", value);
        }
        Result::Err(e) => {
            println!("Oh no you get into this error: {}", e);
        }
    }
    match validate_email("jaga@gmail.com".to_string()) {
        Result::Ok(..) => {
            println!("Its an valid email");
        }
        Result::Err(value) => {
            println!("{:#?}", value);
        }
    }
}
