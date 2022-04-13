// use std::io;
use std::process;
mod bioinformatics_stronghold;

fn main() {
    if let Err(e) = bioinformatics_stronghold::mprt::mprt("inputs/mprt.txt"){
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
