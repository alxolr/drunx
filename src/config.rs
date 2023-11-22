use std::path::PathBuf;

pub fn lookable_files(path: &PathBuf) -> Vec<PathBuf> {
    let files = vec![
        path.join(PathBuf::from("package.json")),
        path.join(PathBuf::from("package-lock.json")),
        path.join(PathBuf::from("src/version.json")),
        path.join(PathBuf::from("version.json")),
    ]
    .into_iter()
    .filter(|file: &PathBuf| file.exists())
    .collect();

    files
}
