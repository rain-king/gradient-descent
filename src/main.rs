//#![allow(warnings)]
use std::io::stdin;
//use std::env;
mod gradient_descent;
use gradient_descent::gradient_descent;

fn main() {
	// env::set_var("RUST_BACKTRACE", "1");
	// println!("{}\n{}", "Write 1 if this is a maximization problem,",
	// 	"or 0 if it's a minimization problem, then press Return again.");
	// let maximize = read_bool();
	
	println!("Write the variables in which f: R^n -> R to optimize is given:");
	let variables = read_string_vec_sorted();

	println!("Write the function x -> f(x) to optimize:");
	let str_f = read_string();
	
	println!("Give an initial guess to the solution:");
	let initial_guess = read_vec_f64();

	let f = exmex::parse::<f64>(&str_f).expect("Couldn't parse string to algebra system.");
	
	println!("{f}");
	
	let (x, fx) = gradient_descent(f, variables, initial_guess, 0.8, 10000);
	
	println!("The optimal x value is {:?}\nwith value of {}.", x, fx);
}

fn read_string() -> String {
	let mut line = String::new();
	stdin().read_line(&mut line).unwrap();
	line.trim().to_string()
}

fn read_string_vec_sorted() -> Vec<String> {
	let mut line = String::new();
	stdin().read_line(&mut line).unwrap();
	let mut vec: Vec<String>;
	vec = line.split_whitespace().map(|s| s.trim().to_string()).collect();
	vec.sort();
	vec
}

fn read_vec_f64() -> Vec<f64> {
	let mut line = String::new();
	stdin().read_line(&mut line).unwrap();
	
	line.split_whitespace().map(|s| s.parse::<f64>().unwrap()).collect()
}
