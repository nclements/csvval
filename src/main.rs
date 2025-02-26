use clap::Parser;
use csv::ReaderBuilder;
use glob::glob;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// CSV filenames (including wildcards)
    #[arg(value_name = "FILENAMES")]
    filenames: Vec<String>,
}

const EXIT_FAILURE: i32 = 1;

fn main() {
    let args = Args::parse();

    if args.filenames.is_empty() {
        eprintln!("No filenames provided.");
        process::exit(EXIT_FAILURE);
    }

    match process_csv_files(args.filenames) {
        Ok(_) => println!("All files are valid CSVs."),
        Err(errors) => {
            for error in errors {
                eprintln!("{}", error);
            }
            process::exit(EXIT_FAILURE);
        }
    }
}

fn process_csv_files(filenames: Vec<String>) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    for filename in filenames {
        println!("{}", filename);
        match glob(&filename) {
            Ok(files) => {
                for file in files {
                    match file {
                        Ok(path) => {
                            let filepath = path.to_string_lossy();
                            if let Err(e) = validate_csv_file(&filepath) {
                                errors.push(format!("Failed to load {}: {}", filepath, e));
                            }
                        }
                        Err(e) => {
                            errors.push(format!("Error reading file {}: {}", filename, e));
                        }
                    }
                }
            }
            Err(e) => {
                errors.push(format!("Invalid file pattern {}: {}", filename, e));
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_csv_file(filename: &str) -> Result<(), csv::Error> {
    let mut csv_reader = ReaderBuilder::new().from_path(filename)?;
    for result in csv_reader.records() {
        let _record = result?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;
