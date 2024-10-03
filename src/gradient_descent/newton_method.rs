use exmex::{Differentiate, Express, FlatEx};

use crate::gradient_descent::THRESHOLD;

fn newton_method(f: FlatEx<f64>, initial_guess: f64) -> f64 {
	let mut i: usize = 1;
	let df = f.clone().partial(0).unwrap();
	let ddf = df.clone().partial(0).unwrap();
	let mut df_x = df.eval(&[initial_guess]).unwrap();
	let mut ddf_x = ddf.eval(&[initial_guess]).unwrap();
	let mut x1 = initial_guess;
	let mut x2 = initial_guess + 1.0;
	
	loop {
		if ddf_x.abs() <= THRESHOLD {
			println!("Second derivative is 0 at x_{}, stopping.", i+1);
			break;
		}
		
		print!("Iteration {i}");
		let next_value = x1 - df_x / ddf_x;
		if i > 1 {
			x1 = x2;
		}
		x2 = next_value;
		
		if (x2 - x1).abs() < THRESHOLD {
			println!("Difference below threshold. Stopping.");
			break;
		}
		
		df_x = df.eval(&[x2]).unwrap();
		ddf_x = ddf.eval(&[x2]).unwrap();
		
		i += 1;
	}
	
	x2
}
