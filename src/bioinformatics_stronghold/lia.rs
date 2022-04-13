use std::error::Error;
use std::fs;
use statrs::distribution::{Binomial, Discrete, DiscreteCDF};

#[allow(dead_code)]
pub fn lia(filename :&str) -> Result<(), Box<dyn Error>> {
	let input = fs::read_to_string(filename)?;
	let mut input = input.split_whitespace();
	let k:u32 = input.next().unwrap().parse().expect("please input number");
	let n:u64 = input.next().unwrap().parse().expect("please input number");

	let binom = Binomial::new(0.25, 2_u64.pow(k)).unwrap();
	let cdf = binom.cdf(n-1);
	println!("{}", 1.0-&cdf);

	Ok(())
}