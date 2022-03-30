use std::io;
use rosalind_rs::string_algorithms::rna;

fn main() -> io::Result<()>{
    rna::solve("inputs/rna.txt")
}
