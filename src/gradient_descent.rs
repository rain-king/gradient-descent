use exmex::{eval_str, parse, Differentiate, Express, FlatEx};
mod newton_method;
const THRESHOLD: f64 = 1E-12;

pub fn gradient_descent(f: FlatEx<f64>, variables: Vec<String>, initial_guess: Vec<f64>, beta: f64, max_iterations: usize)
-> (Vec<f64>, f64) {
	let mut k: usize = 1;
	let mut x = initial_guess;
	let mut gamma: f64 = 0.0001;
	
	loop {
		if k > max_iterations {
			panic!("Max iterations reached, algorithm failed.");
		}
		println!("Iteration {k}");
		let gradfx: Vec<f64> = get_gradient(&f, &variables)
			.iter()
			.map(|df| df.eval(&x).unwrap())
			.collect();
		
		let new_x: Vec<f64> = x.iter().zip(&gradfx).map(|(xj, gradj)| xj - gamma*gradj).collect();
		let x_minus_new_x: Vec<f64> = x.iter().zip(&new_x).map(|(xj, new_xj)| xj - new_xj).collect();
		if norm(&x_minus_new_x) < THRESHOLD {
			println!("No further improvement noticed, stopping the algorithm succesfully.");
			break;
		} else {
			x = new_x;
		}
		
		if norm(&gradfx) < THRESHOLD {
			println!("Optimal solution reached.");
			break;
		}
		
		dbg!(&x, f.eval(&x).unwrap());
		gamma *= beta;
		k += 1;
	}
	
	(x.clone(), f.eval(&x).unwrap())
}

fn get_gradient(f: &FlatEx<f64>, variables: &Vec<String>) -> Vec<FlatEx<f64>> {
	let mut gradient: Vec<FlatEx<f64>> = Vec::new();
	
	for i in 0..variables.len() {
		gradient.push(f.clone().partial(i).unwrap());
	}
	gradient
}

fn norm(x: &Vec<f64>) -> f64 {
	x.iter()
		.map(|&value| value.abs())
		.sum()
}
