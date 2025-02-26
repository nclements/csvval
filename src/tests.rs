use super::*;
use std::io::{self, Write};
use tempfile::NamedTempFile;

fn create_temp_csv_file(records: Vec<Vec<&str>>) -> io::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    {
        let writer = file.as_file_mut();
        for record in records {
            writeln!(writer, "{}", record.join(","))?;
        }
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
