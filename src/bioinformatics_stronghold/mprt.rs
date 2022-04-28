use std::error::Error;
use std::fs;
use curl::easy::Easy;
use needletail::parse_fastx_reader;

#[allow(dead_code)]
pub fn mprt(filename :&str) -> Result<(), Box<dyn Error>> {
	let url_prefix = "https://www.uniprot.org/uniprot/";
	let input = fs::read_to_string(filename)?;
	let mut input_lines = input.lines();
	while let Some(uniprot_id) = input_lines.next() {
		let url = format!("{}{}.fasta", url_prefix, uniprot_id.trim().to_string());
		// println!("{:?}", url);
		let mut dst = Vec::new();
		let mut easy = Easy::new();
		easy.url(&url).unwrap();
		easy.follow_location(true).unwrap(); // allow redirect
		{
			let mut transfer = easy.transfer();
			transfer.write_function(|data| {
				dst.extend_from_slice(data);
				Ok(data.len())
			}).unwrap();
			transfer.perform().unwrap();
		}
		let seq_input:&[u8] = &dst;
		let mut fastx_reader = parse_fastx_reader(seq_input).expect("invalid reader");
		while let Some(r) = fastx_reader.next() {
			let record = r.expect("invalid record");
			let seq = record.seq();
			let mut pos = vec![];
			for i in 0..(seq.len()-3){
				if seq[i]==b'N' {
					if seq[i+1]!=b'P' {
						if seq[i+2]==b'S' || seq[i+2]==b'T' {
							if seq[i+3]!=b'P' {
								pos.push(i+1);
							}
						}
					}
				}
			}
			if pos.len()!=0{
				println!("{}", uniprot_id);
				pos.iter().for_each(|x|print!("{} ", x));
				println!("");

			}
		}
	}

	Ok(())
}