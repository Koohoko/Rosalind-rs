use std::error::Error;
// use rosalind_rs::bioinformatics_stronghold;
use rosalind_rs::bioinformatics_textbook;

fn main() -> Result<(), Box<dyn Error>> {
    bioinformatics_textbook::ba1d::solve("inputs/ba1d.txt")
}
