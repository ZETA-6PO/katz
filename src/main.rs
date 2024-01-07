use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("usage: katz [-H --help] [-N --number] [-C --compressed] <file>");
        return;
    }

    //all the options
    let mut help = false;
    let mut endline = false;
    let mut number = false;
    let mut compressed = false;

    //the file path, I use Option in order to manage the case of an non-specified path
    let mut path: Option<PathBuf> = None;

    for arg in &args[1..] {
        match arg.as_str() {
            "-H" | "--help" => {
                //Help option
                help = true;
            }
            "-E" | "--endline" => {
                //End line option
                endline = true;
            }
            "-N" | "--number" => {
                //With line number option
                number = true;
            }
            "-C" | "--compressed" => {
                //Compressed option (skip blank line)
                compressed = true;
            }
            _ => {
                //check if its opt
                if arg.starts_with("-") {
                    println!("error: unrecognized option {arg}");
                    std::process::exit(1);
                }
                //Path option
                if path.is_some() {
                    println!("error: you should only provide a single path");
                    std::process::exit(1);
                } else {
                    path = Some(PathBuf::from(arg));
                }
            }
        }
    }

    //check for help option
    if help {
        println!("help");
        std::process::exit(0);
    }

    //check if path is not None
    if path.is_none() {
        println!("error: you must provide a path");
        std::process::exit(0);
    }

    //check if path is correct
    if path.clone().unwrap().as_path().is_dir() {
        println!("error: path is a directory");
        std::process::exit(0);
    }

    //try to open the file
    let file = match File::open(path.clone().unwrap().as_path()) {
        Ok(f) => f,
        Err(err) => {
            println!(
                "error: cannot open {}, {}",
                path.unwrap().display(),
                err.to_string()
            );
            std::process::exit(1);
        }
    };

    // Print action

    let mut blc = 0u8; //blank line count

    let buffer = BufReader::new(file);

    for (index, line) in buffer.lines().enumerate() {
        let cl = match line {
            Ok(f) => f,
            Err(err) => {
                println!("error: cannot print file, {}", err.to_string());
                std::process::exit(1);
            }
        };

        if cl.is_empty() {
            blc += 1;
        }

        // if compressed mode activated dont print more than 2 consecutives empty line
        if blc >= 2 && compressed {
            blc = 0;
            continue;
        }

        if number {
            print!("{} ", index);
        }

        print!("{}", cl);

        if endline {
            println!("$");
        } else {
            println!();
        }
    }
}
