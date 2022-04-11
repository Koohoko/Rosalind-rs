use std::error::Error;
use std::fs;

#[allow(dead_code)]
pub fn iprb(filename :&str) -> Result<(), Box<dyn Error>> {

	let reader = fs::read_to_string(filename)?;
	let input: Vec<&str> = reader.split(" ").collect();
	let k:usize = input[0].trim().parse()?; // homozygous
	let m:usize = input[1].trim().parse()?; // heterozygous
	let n:usize = input[2].trim().parse()?; // homozygous recessive
	
	//Return: The probability that two randomly selected mating organisms will produce an individual possessing a dominant allele.
	let total = k + m + n;
	let prob_senario_1 = minor_from_diff(m, n, total)*0.5;
	let prob_senario_2 = minor_from_diff(n, m, total)*0.5;
	let prob_senario_3 = minor_from_same( m, total)*0.25;
	let prob_senario_4 = minor_from_same( n, total);

	let total_prob_minor = prob_senario_1+prob_senario_2+prob_senario_3+prob_senario_4;

	println!("{}", 1.0-total_prob_minor);
	Ok(())
}

fn minor_from_diff(pop1:usize, pop2:usize, total:usize) -> f64 {
	let pop1 = pop1 as f64;
	let pop2 = pop2 as f64;
	let total = total as f64;
	(pop1/total) * ((pop2)/(total-1.0))
}

fn minor_from_same(pop1:usize, total:usize) -> f64 {
	let pop1 = pop1 as f64;
	let total = total as f64;
	(pop1/total) * ((pop1-1.0)/(total-1.0))
}