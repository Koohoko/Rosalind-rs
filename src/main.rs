use std::error::Error;
// use rosalind_rs::bioinformatics_stronghold;
// use rosalind_rs::bioinformatics_textbook;
use rosalind_rs::algorithmic_heights;

fn main() -> Result<(), Box<dyn Error>> {
    // bioinformatics_stronghold::trans::solve("inputs/trans.txt")
    algorithmic_heights::fibo::solve("inputs/fibo.txt")
}
