/*
 *  This program extracts headers from a java file
 */
use clap::{Arg, App};
use std::process::exit;
use rust_srcutils::extract_java_headers;

fn main() {
    let matches = App::new("Java source header extractor")
        .version("0.1.0")
        .author("Moisès Gómez Girón <moiatgit@gmail.com>")
        .about("Extracts the header comments of a java source file.")
        .arg(Arg::new("file")
            .about("Java source file")
            .required(true))
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    if ! filename.ends_with(".java") {
        println!("ERROR: A java source file was expected");
        exit(exitcode::DATAERR);
    }

    if ! std::path::Path::new(filename).exists() {
        println!("ERROR: file not found {}", filename);
        exit(exitcode::NOINPUT);
    }

    match std::fs::read_to_string(filename) {
        Ok(contents) => {
            let result = extract_java_headers(&contents);
            println!("{}", result);
        },
        Err(e) => {
            println!("ERROR: problems reading file: {}", e);
            exit(exitcode::DATAERR);
        }
    };
}

