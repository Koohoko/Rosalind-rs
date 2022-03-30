use std::io;
use rosalind_rs::string_algorithms::revc;

fn main() -> io::Result<()>{
    revc::solve("inputs/revc.txt")
}
