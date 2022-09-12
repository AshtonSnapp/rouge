//--> Imports <--

use std::{
	path::Path,
};

use clap::{};

//--> Type Aliases <--

//--> Structs <--

//--> Enums <--

//--> Traits <--

//--> Functions <--

fn main() {
	let args = clap::command!()
		.long_about(
			""
		)
		.get_matches();
}