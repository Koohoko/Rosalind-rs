use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[allow(dead_code)]
pub fn mrna(filename :&str) -> Result<(), Box<dyn Error>> {
	let inv_codon_table = "A	GCT, GCC, GCA, GCG
	R	CGT, CGC, CGA, CGG; AGA, AGG
	N	AAT, AAC
	D	GAT, GAC
	B	AAT, AAC; GAT, GAC
	C	TGT, TGC
	Q	CAA, CAG
	E	GAA, GAG
	Z	CAA, CAG; GAA, GAG
	G	GGT, GGC, GGA, GGG
	H	CAT, CAC
	I	ATT, ATC, ATA
	L	CTT, CTC, CTA, CTG; TTA, TTG
	K	AAA, AAG
	M	ATG
	F	TTT, TTC
	P	CCT, CCC, CCA, CCG
	S	TCT, TCC, TCA, TCG; AGT, AGC
	T	ACT, ACC, ACA, ACG
	W	TGG
	Y	TAT, TAC
	V	GTT, GTC, GTA, GTG
	*	TAA, TGA, TAG";
	let inv_codon_table: Vec<_> = inv_codon_table.lines().collect();
	let mut inv_codon_hm = HashMap::new();
	for str in inv_codon_table {
		let split: Vec<_> = str.trim().split("\t").collect();
		let mut num_codon = split[1].matches(",").count();
		num_codon += split[1].matches(";").count()+1;
		// println!("{}", split[1]);
		// println!("{}", num_codon);
		inv_codon_hm.insert(split[0].to_string(), num_codon);
	}

	let input = fs::read_to_string(filename)?;
	let mut result:usize=1;
	for aa in input.trim().chars(){
		// dbg!(aa);
		result = result * inv_codon_hm.get(&aa.to_string()).expect("invalid codon");
		result = result % 1_000_000;
	}

	println!("{}", result*3%1_000_000);
	Ok(())
}