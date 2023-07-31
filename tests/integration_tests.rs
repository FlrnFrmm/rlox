use eyre::Result;
use std::fs::DirEntry;
use std::io::Error;

fn read_test_file_contents(test_data_dir: &str) -> Result<Vec<String>> {
    std::fs::read_dir(test_data_dir)?
        .map(read_file_contents)
        .collect::<Result<Vec<String>>>()
}

fn read_file_contents(file_entry: Result<DirEntry, Error>) -> Result<String> {
    let file_content = std::fs::read_to_string(file_entry?.path())?;
    Ok(file_content)
}
