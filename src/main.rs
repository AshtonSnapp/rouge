//--> Imports <--

use std::{
	path::PathBuf,
};

use clap::{
	Arg,
	Command,
	ValueHint,
};

//--> Type Aliases <--

//--> Structs <--

//--> Enums <--

//--> Traits <--

//--> Functions <--

fn main() {
	let args = Command::new("rouge")
		.version(clap::crate_version!())
		.about("A rusty programming and scripting language for applications.")
		.long_about(
			"Rouge is a statically-typed programming language desigend for general programming and scripting. It takes inspiration from Rust, Ruby, and Python.\
			This utility acts as a way for you to both compile and interpret Rouge code.\
			Rouge code passed in as arguments will be compiled to bytecode and immediately interpreted.\
			With the -C/--compile option, the bytecode will instead be written to a file.\
			Want both? Use -C/--compile in combination with -r/--run (which can only be used with -C/--compile)."
		)
		.arg_required_else_help(true) // Eventually I want to have a REPL / interactive session, but I don't know how to implement that yet.
		.arg(
			Arg::new("compile")
			.short('C')
			.long("compile")
			.help("Compile the code and save it in a file.")
			.long_help(
				""
			)
		)
		.arg(
			Arg::new("run")
			.short('r')
			.long("run")
			.requires("compile")
			.help("")
			.long_help(
				""
			)
		)
		.arg(
			Arg::new("outfile")
			.short('o')
			.long("out")
			.requires("compile")
			.value_name("OUTFILE")
			.value_parser(clap::value_parser!(PathBuf))
			.num_args(1)
			.help("")
			.long_help(
				""
			)
		)
		.arg(
			Arg::new("files")
			.required(true)
			.value_name("FILES")
			.value_parser(clap::value_parser!(PathBuf))
			.num_args(1..)
			.help("")
			.long_help(
				""
			)
		)
		.get_matches();
}