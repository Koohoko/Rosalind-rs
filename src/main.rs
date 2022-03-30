use std::io;
use rosalind_rs::string_algorithms::dna;

fn main() -> io::Result<()>{
    dna::solve("inputs/dna.txt")
}
