use std::error::Error;
use needletail::{parse_fastx_file};

#[allow(dead_code)]
pub fn cons(filename :&str) -> Result<(), Box<dyn Error>> {
	let mut reader = parse_fastx_file(filename)?;
	let seq = reader.next().unwrap()?.seq().to_ascii_uppercase();
	let seq_len = seq.len();
	let mut a_vec = vec![0;seq_len];
	let mut c_vec = vec![0;seq_len];
	let mut g_vec = vec![0;seq_len];
	let mut t_vec = vec![0;seq_len];
	for (i, base) in seq.into_iter().enumerate(){
		match base {
			b'A' => a_vec[i] +=1,
			b'C' => c_vec[i] +=1,
			b'G' => g_vec[i] +=1,
			b'T' => t_vec[i] +=1,
			_ => ()
		}
	}

	while let Some(record) = reader.next() {
        let seqrec = record?;
		let seq = seqrec.seq().to_ascii_uppercase();
		for (i, base) in seq.into_iter().enumerate(){
			match base {
				b'A' => a_vec[i] +=1,
				b'C' => c_vec[i] +=1,
				b'G' => g_vec[i] +=1,
				b'T' => t_vec[i] +=1,
				_ => ()
			}
		}		
    }

	let mut consensus = Vec::new();
	for i in 0..seq_len { // consensus
		let consensus_i = vec![a_vec[i], c_vec[i], g_vec[i], t_vec[i]]
		.iter()
		.enumerate()
		.max_by(|(_, a), (_, b)|a.cmp(b))
		.map(|(idx, _)|match idx {
			0 => b'A',
			1 => b'C',
			2 => b'G',
			3 => b'T',
			_ => b'N',
		});
		consensus.push(consensus_i.unwrap());
	}
	println!("{}", String::from_utf8(consensus).unwrap());
	print!("A: ");
	for cnt in a_vec {
		print!("{} ", cnt);
	}
	print!("\n");
	print!("C: ");
	for cnt in c_vec {
		print!("{} ", cnt);
	}
	print!("\n");
	print!("G: ");
	for cnt in g_vec {
		print!("{} ", cnt);
	}
	print!("\n");
	print!("T: ");
	for cnt in t_vec {
		print!("{} ", cnt);
	}
	print!("\n");

	Ok(())
}
