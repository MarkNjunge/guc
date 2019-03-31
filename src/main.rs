use std::env;
use std::process;

mod convert;

fn main() {
    let url = env::args().nth(1).unwrap_or_else(|| {
        println!("URL is required as argument!");
        process::exit(1);
    });

    let converted = convert::run(url);

    println!("{}", converted);
}
