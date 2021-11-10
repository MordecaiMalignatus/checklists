#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]

use clap::{App, Arg};
use std::path::{Path, PathBuf};
use std::io::BufRead;
use anyhow::Result;

fn main() -> Result<()> {
    let mut app = App::new("cls")
        .version("0.1")
        .author("Mordecai Malignatus <mordecai@malignat.us>")
        .about("Tool to step through checklists.")
        .arg(
            Arg::with_name("FILE")
                .takes_value(true)
                .required(false)
                .short("f")
                .long("file"),
        );

    let args = app.clone().get_matches();
    match args.value_of("FILE") {
        Some(file) => run_from_file(&PathBuf::from(file)),
        None => {
            if isatty::stdin_isatty() {
                eprintln!("Pass something to stdin or supply a file in order to step through.\n");
                app.print_help()?;

                Ok(())
            } else {
                run_from_stdin()
            }
        }
    }
}

fn run_from_file(path: &Path) -> Result<()> {
    let lines = slurp_file(path).lines().map(|s| s.into()).collect();
    let filename = path.file_name().unwrap().to_string_lossy();

    stepper_lib::step_list(stepper_lib::List::new(lines, filename.to_string()))
}

fn slurp_file(p: &Path) -> String {
    match std::fs::read_to_string(p) {
        Ok(x) => x,
        Err(e) => panic!("Can't read string at path {}: {}", p.display(), e),
    }
}

fn run_from_stdin() -> Result<()> {
    let handle = std::io::stdin();
    let lock = handle.lock();
    let lines: Vec<_> = lock.lines().map(Result::unwrap).collect();

    stepper_lib::step_list(stepper_lib::List::new(
        lines,
        String::from("Piped from STDIN."),
    ))
}
