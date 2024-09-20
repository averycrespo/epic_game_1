use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    print_something();

    return Ok(());
}

fn print_something() {
    println!("Hello Avery!");
}
