use std::error::Error;
use needletail::parse_fastx_file;
use needletail::kmer;

#[allow(dead_code)]
pub fn lcsm(filename :&str) -> Result<(), Box<dyn Error>> {
	let mut seq_lens = Vec::new();
	let mut seqs = Vec::new();
	let mut reader = parse_fastx_file(filename)?;
	while let Some(record) = reader.next(){
		let seq_i = record.expect("invalid record").seq().into_owned();
		seq_lens.push(seq_i.len());
		seqs.push(seq_i);
	}

	// find shortest seq
	let smallest = seq_lens
	.iter()
	.enumerate()
	.min_by(|(_, val1), (_, val2)|val1.cmp(val2))
	.unwrap();

	let seq_min_idx = smallest.0;
	let seq_min_len = smallest.1;
	let seq_min = seqs[seq_min_idx].to_owned();
	seqs.remove(seq_min_idx);

	// binary search
	let mut left:usize = 0;
	let mut right:usize = *seq_min_len+1;
	let mut mid ;

	let mut final_kmer = b"".to_vec();
	while left + 1 < right {
		mid = (left + right) / 2;
		println!("L:{}, R:{}, M:{}", left, right, mid);

		let tmp = common_substr(&seq_min, &seqs, mid);
		if tmp.0{
			final_kmer = tmp.1;
			left = mid;
		} else {
			right = mid;
		}
	}

	println!("{}", String::from_utf8(final_kmer).unwrap());
	Ok(())
}

fn common_substr(seq_0:&Vec<u8>, seqs:&Vec<Vec<u8>>, k:usize) -> (bool, Vec<u8>) {
	let k_iter = kmer::Kmers::new(seq_0, k as u8);
	let mut kmer_found=(false, b"".to_vec());
	for kmer in k_iter {
		let kmer_str = std::str::from_utf8(kmer).unwrap();
		let mut check_found = false;
		for seq_i in seqs.iter(){
			let seq_i_str = std::str::from_utf8(seq_i).unwrap();
			if seq_i_str.contains(kmer_str) {
				check_found = true;
			} else {
				check_found = false;
				break;
			}
		}
		if check_found {
			kmer_found.0 = true;
			kmer_found.1 = kmer.to_owned();
			break;
		} else {
			continue;
		}
	}
	kmer_found
}
