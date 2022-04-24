// use std::io;
use std::process;
mod bioinformatics_stronghold;

fn main() {
    if let Err(e) = bioinformatics_stronghold::mrna::mrna("inputs/mrna.txt"){
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
