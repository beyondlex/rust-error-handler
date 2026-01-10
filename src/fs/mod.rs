

pub fn list_files(path: &str) -> crate::Result<Vec<String>> {
    let files: Vec<String> = std::fs::read_dir(path)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect()
        ;
    if files.is_empty() {
        return Err("No files found".into());
    }
    Ok(files)
}
