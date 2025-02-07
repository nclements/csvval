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
mod tests {
    use super::*;
    use csv::Writer;
    use std::io::{self, Result as IoResult, Write};
    use tempfile::NamedTempFile;

    fn create_temp_csv_file(records: Vec<Vec<&str>>) -> io::Result<NamedTempFile> {
        let mut file = NamedTempFile::new()?;
        {
            let mut writer = Writer::from_writer(&file);
            for record in records {
                writer.write_record(record)?;
            }
            writer.flush()?;
        }
        Ok(file)
    }

    #[test]
    fn test_validate_csv_file_success() {
        let records = vec![vec!["header1", "header2"], vec!["value1", "value2"]];
        let file = create_temp_csv_file(records).unwrap();
        let path = file.path().to_string_lossy().into_owned();
        assert!(validate_csv_file(&path).is_ok());
    }

    #[test]
    fn test_validate_csv_file_failure() {
        let records = vec![
            vec!["header1", "header2"],
            vec!["value1", "value2", "value3"],
        ];
        let file = create_temp_csv_file(records).unwrap();
        let path = file.path().to_string_lossy().into_owned();
        assert!(validate_csv_file(&path).is_err());
    }

    #[test]
    fn test_process_csv_files_success() {
        let records1 = vec![vec!["header1", "header2"], vec!["value1", "value2"]];
        let records2 = vec![vec!["header3", "header4"], vec!["value3", "value4"]];
        let file1 = create_temp_csv_file(records1).unwrap();
        let file2 = create_temp_csv_file(records2).unwrap();
        let path1 = file1.path().to_string_lossy().into_owned();
        let path2 = file2.path().to_string_lossy().into_owned();
        assert!(process_csv_files(vec![path1, path2]).is_ok());
    }

    #[test]
    fn test_process_csv_files_failure() {
        let records1 = vec![vec!["header1", "header2"], vec!["value1", "value2"]];
        let records2 = vec![
            vec!["header3", "header4"],
            vec!["value3", "value4", "value5"],
        ];
        let file1 = create_temp_csv_file(records1).unwrap();
        let file2 = create_temp_csv_file(records2).unwrap();
        let path1 = file1.path().to_string_lossy().into_owned();
        let path2 = file2.path().to_string_lossy().into_owned();
        let result = process_csv_files(vec![path1, path2]);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_process_csv_files_multiple_failures() {
        let records1 = vec![
            vec!["header1", "header2"],
            vec!["value1", "value2", "value3"],
        ];
        let records2 = vec![
            vec!["header3", "header4"],
            vec!["value3", "value4", "value5"],
        ];
        let file1 = create_temp_csv_file(records1).unwrap();
        let file2 = create_temp_csv_file(records2).unwrap();
        let path1 = file1.path().to_string_lossy().into_owned();
        let path2 = file2.path().to_string_lossy().into_owned();
        let result = process_csv_files(vec![path1, path2]);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2);
    }
}
