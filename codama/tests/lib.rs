mod membership;
mod system;

pub fn get_path(relative_path: &str) -> std::path::PathBuf {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    std::path::Path::new(project_dir)
        .join("tests")
        .join(relative_path)
}
