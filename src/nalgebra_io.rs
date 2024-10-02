//#![allow(warnings)]
mod vec_io;
use vec_io::read_vec;
use std::io::{stdin, Read};
use na::{SMatrix, RowSVector};

pub fn read_matrix(message: &str) -> SMatrix<f64> {
	if !message.is_empty() {
		println!("{message}");
	}

	let mut a_rows = Vec::new();
	let mut row = Vec::new();
	loop {
		row = read_vec("");
		if row.is_empty() {
			break;
		}
		a_rows.push(RowSVector::from_vec(row));
	}
	
	SMatrix::from_rows(&a_rows)
}
