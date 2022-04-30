use indexmap::IndexMap;
use std::error::Error;
use std::fs::read_to_string;
use needletail::Sequence;
// use std::num::Wrapping;

#[allow(dead_code)]
pub fn solve(file_path: &str) -> Result<(), Box<dyn Error>>{
	let input = read_to_string(file_path)?;
	let mut input = input.lines();
	let seq = input.next().unwrap().as_bytes();
	let line2 = input.next().unwrap();
	let line2 = line2.trim().split_ascii_whitespace().collect::<Vec<&str>>();
	let k:usize = line2[0].parse()?;
	let l:usize = line2[1].parse()?;
	let t:usize = line2[2].parse()?;
	// dbg!([k, l, t]);
	let results = ba1e(seq, k, l, t);

	for result in results {
		print!("{} ", String::from_utf8(result)?);
	}
	print!("\n");

	Ok(())
}

fn ba1e(seq:&[u8], k:usize, l:usize, t:usize) -> Vec<Vec<u8>>{
	let mut results = Vec::new();
	let kmers = seq.kmers(k as u8);
	let mut id = 0_usize;
	let mut hm_kmer_found:IndexMap<&[u8], Vec<usize>> = IndexMap::new();
	for kmer in kmers {
		let value = hm_kmer_found.entry(kmer).or_insert(vec![]);
    	value.push(id);
		id += 1;
	}

	for (k, v) in hm_kmer_found{
		if v.len()<t {continue;}
		// print!("{} ", String::from_utf8(k.to_vec()).unwrap());
		for i in (t-1)..v.len() {
			if v[i]-v[i-t+1]+t <= l {
				results.push(k.to_vec());
				break;
			}
		}
	}
	// dbg!(&results);ca

	results

}