use cli::get_cli_arguments;
use csv::Error;

pub mod cli;



pub fn run() -> Result<(), Error> {
    let cli_arguments = get_cli_arguments(std::env::args_os());
    println!("Arguments: {:?}", cli_arguments);
    Ok(())
}


fn main() {
    match run() {
       Ok(_) => {}
       Err(_) => {
            std::process::exit(1);
       } 
    }
    println!("Hello, world!");
}
