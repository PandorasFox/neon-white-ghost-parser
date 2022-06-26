use std::io::Read;
use std::io;

mod ghost;
mod ghost_parser;
mod ghost_stats;

fn main() {
    let mut ghost = ghost::Ghost::new();
    let mut buffer = String::new();
    match io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {
            if let Err(error) = ghost_parser::parse(&mut ghost, &buffer) {
                eprintln!("error: {error}");
                std::process::exit(1);
            }
            // process results here! We have it all in-memory!
            ghost_stats::collect_stats(&ghost);
        }
        Err(error) => println!("error: {error}"),
    }
}
