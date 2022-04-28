use std::error::Error;
// mod bioinformatics_stronghold;
mod bioinformatics_textbook;

fn main() -> Result<(), Box<dyn Error>> {
    bioinformatics_textbook::ba1b::ba1b("inputs/ba1b.txt")
}
