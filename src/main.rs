//--> Imports <--

mod runtime;

use clap::{Arg, Command, command, ValueHint};
use std::path::Path;
use runtime::{Runtime, CompileOptions};
use std::str::FromStr;
use std::process::exit;

//--> Functions <--

fn main() {
    let args = command!()
        .propagate_version(true)
        .trailing_var_arg(true)
        .subcommand(Command::new("compile")
            .short_flag('C')
            .long_flag("compile")
            .about("Compile Rouge into bytecode objects.")
            .trailing_var_arg(true)
            .arg(Arg::new("optimize")
                .short('O')
                .long("opt")
                .value_name("OPT-LEVEL")
                .help("Sets the optimization level. Defaults to 0 (no optimizations).")
            )
            .arg(Arg::new("zstd-level")
                .short('z')
                .long("zstd")
                .value_name("ZSTD-LEVEL")
                .help("Sets the Zstandard compression level of the output binary. Set to 0 to disable compression.")
            )
            .arg(Arg::new("run")
                .short('r')
                .long("run")
                .help("Also runs the program after it is compiled.")
            )
            .arg(Arg::new("warn-is-err")
                .short('w')
                .long("werr")
                .help("Causes the compiler to treat all warnings as errors.")
            )
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .value_hint(ValueHint::FilePath)
                .allow_invalid_utf8(true)
                .value_name("OUTFILE")
                .help("The compiler will put the bytecode file at this path, overwriting whatever was already there. The given file extension will be ignored/overwritten.")
            )
            .arg(Arg::new("include")
                .short('i')
                .long("include")
                .value_hint(ValueHint::DirPath)
                .allow_invalid_utf8(true)
                .value_name("DIR")
                .multiple_occurrences(true)
                .help("Allows the compiler to search the given directory for included packages and modules.")
            )
            .arg(Arg::new("INFILES")
                .required(true)
                .value_hint(ValueHint::FilePath)
                .allow_invalid_utf8(true)
                .multiple_values(true)
                .help("The files to compile together into bytecode.")
            )
        )
        .arg(Arg::new("include")
            .short('i')
            .long("include")
            .value_hint(ValueHint::DirPath)
            .allow_invalid_utf8(true)
            .value_name("DIR")
            .multiple_occurrences(true)
            .requires("PROGRAM")
            .help("Allows the runtime to search the given directory for included packages and modules.")
        )
        .arg(Arg::new("PROGRAM")
            .value_hint(ValueHint::FilePath)
            .allow_invalid_utf8(true)
            .help("The Rouge program to run. Omit to run in interactive mode (Read-Eval-Print-Loop).")
        )
        .arg(Arg::new("VARGS")
            .value_hint(ValueHint::Other)
            .multiple_values(true)
            .requires("PROGRAM")
            .help("Arguments to be passed to the Rouge program.")
        )
        .get_matches();
    
    let mut rte = Runtime::new();
    
    match args.subcommand() {
        Some((subcmd, subargs)) => match subcmd {
            "compile" => {
                // To start, let's get the names of our input files. This is a required argument, so unwrapping is okay here.
                let infiles = subargs.values_of_os("INFILES").unwrap().map(|x| Path::new(x)).collect::<Vec<&Path>>();

                // Ensure that the infiles are actually files.
                for infile in &infiles {
                    if !infile.is_file() {
                        eprintln!("<ERR!> {} is not a file.", infile.display());
                        exit(1);
                    }
                }

                // Next, let's get our output file's name. The extension will be replaced in the compile function.
                let outfile = if let Some(o) = subargs.value_of_os("output") {
                    Path::new(o)
                } else {
                    Path::new(infiles[0])
                };

                // Then we get the included directories.
                let includes = match subargs.values_of_os("include") {
                    Some(i) => i.map(|x| Path::new(x)).collect::<Vec<&Path>>(),
                    None => Vec::new()
                };

                // Now we get our compilation options.
                let opts = match (subargs.value_of("optimize"), subargs.value_of("zstd-level"), subargs.is_present("warn-is-err")) {
                    (None, None, false) => CompileOptions::default(),
                    (None, None, true) => CompileOptions::new(0, 20, true),
                    (Some(otxt), None, werr) => {
                        let opt = match u8::from_str(otxt) {
                            Ok(num) => num,
                            Err(_) => 0
                        };
                        CompileOptions::new(opt, 20, werr)
                    },
                    (None, Some(ztxt), werr) => {
                        let zst = match i8::from_str(ztxt) {
                            Ok(num) => num,
                            Err(_) => 20
                        };
                        CompileOptions::new(0, zst, werr)
                    },
                    (Some(otxt), Some(ztxt), werr) => {
                        let opt = match u8::from_str(otxt) {
                            Ok(num) => num,
                            Err(_) => 0
                        };
                        let zst = match i8::from_str(ztxt) {
                            Ok(num) => num,
                            Err(_) => 20
                        };
                        CompileOptions::new(opt, zst, werr)
                    }
                };

                if subargs.is_present("run") {
                    // Compile AND run the code, if compilation succeeds.
                    match rte.compile_and_load(infiles, includes, outfile, opts) {
                        Ok((main_func, wrns)) => {
                            for wrn in wrns {
                                eprintln!("{}", wrn.message());
                            }
                        },
                        Err(errs) => {
                            for err in errs {
                                eprintln!("{}", err.message());
                            }
                            exit(1);
                        }
                    }
                } else {
                    // Just compile the code.
                    match rte.compile(infiles, includes, outfile, opts) {
                        Ok(wrns) => {
                            for wrn in wrns {
                                eprintln!("{}", wrn.message());
                            }
                        },
                        Err(errs) => {
                            for err in errs {
                                eprintln!("{}", err.message());
                            }
                            exit(1);
                        }
                    }
                }
            }
            _ => {}
        },
        None => {
            if args.is_present("PROGRAM") {
                // runtime
                let prg = Path::new(args.value_of_os("PROGRAM").unwrap());
                let includes = match args.values_of_os("include") {
                    Some(i) => i.map(|x| Path::new(x)).collect::<Vec<&Path>>(),
                    None => Vec::new()
                };
                match rte.load(prg, includes) {
                    Ok((main_func, wrns)) => {
                        for wrn in wrns {
                            eprintln!("{}", wrn.message());
                        }
                    },
                    Err(errs) => {
                        for err in errs {
                            eprintln!("{}", err.message());
                        }
                        exit(1);
                    }
                }
            } else {
                // repl
                rte.repl();
            }
        }
    }
}
