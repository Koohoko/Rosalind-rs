use std::error::Error;
use std::fs;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn count_rabbit(filename :&str) -> Result<(), Box<dyn Error>> {
	let mut total = HashMap::new();

	let reader = fs::read_to_string(filename)?;
	let mut input_iter = reader.split_ascii_whitespace();
	let n_max:usize = input_iter.next().expect("please input n and k").trim().parse()?;
	let k:usize = input_iter.next().expect("please input n and k").trim().parse()?;

	total.insert(1, 1); // n = 1; count = 1;
	total.insert(2, 1);// n = 2; count = 1;
	for n in 3..=n_max {
		let count_n = total.get(&(n-2)).unwrap()*k + total.get(&(n-1)).unwrap();
		total.insert(n, count_n);
	}
	println!("{}", total.get(&n_max).unwrap());
	Ok(())
}