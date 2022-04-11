use std::error::Error;
use needletail::{parse_fastx_file};
use std::collections::HashMap;

#[allow(dead_code)]
pub fn count_gc(filename :&str) -> Result<(), Box<dyn Error>> {
	let mut gc_contents = HashMap::new();

	let mut reader = parse_fastx_file(filename)?;

	while let Some(record) = reader.next() {
        let seqrec = record?;
		let seq = seqrec.seq();
		let gc_count = seq
			.iter()
			.filter(|&&b| b == b'G' || b == b'g' || b == b'C' || b == b'c')
			.count();
		let gc_por:f64 = gc_count as f64 / seq.len() as f64 * 100.0;
		
		gc_contents.insert(String::from_utf8(seqrec.id().to_vec()).unwrap(), gc_por);
    }
	let max_entry = gc_contents
	.iter()
	.max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
	.unwrap();
	println!("{}", max_entry.0);
	println!("{}", max_entry.1);
	Ok(())
}