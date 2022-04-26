use std::error::Error;
use needletail::{parse_fastx_file};
use protein_translate::translate;

#[allow(dead_code)]
pub fn splc(filename:&str) -> Result<(), Box<dyn Error>>{
	let mut seqs = Vec::new();
	let mut record = parse_fastx_file(filename)?;

	while let Some(r) = record.next() {
		let seqrec = r?;
		seqs.push(seqrec.seq().to_vec());
	}

	let mut seq_raw = seqs[0].clone();
	for seq_intron in seqs.iter().skip(1) {
		let mut idx_del = Vec::new();
		let len = seq_intron.len();
		if len > seq_raw.len() {break;}
		for i in 0..=(seq_raw.len()-len) {
			if &seq_raw[i..(i+len)] == seq_intron {
				for j in (i..(i+len)).rev(){
					idx_del.push(j);
				}
			}
		}
		for idx in idx_del {
			seq_raw.remove(idx);
		}
	}
	let mut seq_aa = translate(&seq_raw);
	if seq_aa.as_bytes()[seq_aa.len()-1] == "*".as_bytes()[0] {seq_aa.pop();}
	println!("{}", seq_aa);
	
	Ok(())
}