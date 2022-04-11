use std::error::Error;
use std::fs;

#[allow(dead_code)]
pub fn fibd(filename :&str) -> Result<(), Box<dyn Error>> {
	let mut count_small:Vec<usize> = vec![1,0];
	let mut count_big:Vec<usize> = vec![0,1];
	let mut count_rabbit:Vec<usize> = vec![1,1];

	let reader = fs::read_to_string(filename)?;
	let mut input_iter = reader.split_ascii_whitespace();
	let n_max:usize = input_iter.next().expect("please input n and m").trim().parse()?;
	let m:usize = input_iter.next().expect("please input n and m").trim().parse()?;

	if n_max<m {
		for n in 2..n_max {
			let count_n = count_rabbit[n-2] + count_rabbit[n-1];
			count_rabbit.push(count_n);
		}
		println!("{}", count_rabbit.last().unwrap());
	} else {
		for n in 2..n_max {
			if n < m {
				count_small.push(count_big[n-1]);
				count_big.push(count_small[n-1] + count_big[n-1]);
				let count_n = count_small[n] + count_big[n];
				count_rabbit.push(count_n);
			} else {
				count_small.push(count_big[n-1]);
				count_big.push(count_small[n-1] + count_big[n-1] - count_small[n-m]);
				let count_n = count_small[n] + count_big[n];
				count_rabbit.push(count_n);
				// println!("{:?}", count_rabbit);
			}	
		}
		// println!("{:?}", count_small);
		// println!("{:?}", count_big);
		// println!("{:?}", count_rabbit);
		println!("{}", count_rabbit.last().unwrap());
	}
	
	Ok(())
}