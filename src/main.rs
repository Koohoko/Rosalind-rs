use std::error::Error;
mod bioinformatics_stronghold;

fn main() -> Result<(), Box<dyn Error>> {
    bioinformatics_stronghold::lexv::lexv("inputs/lexv.txt")
}
