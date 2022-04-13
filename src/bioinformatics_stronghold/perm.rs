use std::error::Error;
use std::fs;
use std::num::Wrapping;

#[allow(dead_code)]
pub fn perm(filename :&str) -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string(filename)?;
	let n:usize = input.trim().parse().expect("please input a number");
	let n_u8 = n as u8;
	let mut perm_all = Vec::new();
	perm_all.push((1..=n_u8).collect::<Vec<u8>>());

	let mut seq_new ;
	let mut check = true;
	while check {
		seq_new = swap(&perm_all.last().unwrap(), n);
		if &seq_new != perm_all.last().unwrap(){
			perm_all.push(seq_new);
		} else {
			check = false;
		}
	}

	println!("{}", perm_all.len());
	for seq in perm_all {
		seq.iter().for_each(|val| print!("{} ", val));
		print!("\n");
	}

	Ok(())
}

fn swap(seq:&Vec<u8>, n:usize) -> Vec<u8>{
	let mut seq_tmp = seq.clone();
	'outer: for i in (1..n).rev() {
		if seq[i-1] > seq[i] {
			continue 'outer;
		}
		let j = (Wrapping(i) - Wrapping(1)).0;
		let tail_min_big = seq[j..n].iter().enumerate().filter(|(_, &val)|val>seq_tmp[j]).min_by(|(_, a), (_, b)|a.cmp(b)).unwrap();
				seq_tmp[j] = *tail_min_big.1;
				seq_tmp[(j..n).nth(tail_min_big.0).unwrap()] = seq[j];
				let gap = (Wrapping(n-1) - Wrapping(j)).0;
				if gap == 1 {break 'outer;}
				for k in 0..(gap/2) {
					let tmp1 = seq_tmp[j+k+1];
					let tmp2 = seq_tmp[n-1-k];
					seq_tmp[n-1-k] = tmp1;
					seq_tmp[j+k+1] = tmp2;
				}
				break 'outer;
	};

	seq_tmp
}