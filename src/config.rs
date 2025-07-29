use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::path::PathBuf;

/// Searches for the .tudu file starting from the given directory and moving up.
///
/// # Returns
///
/// Returns `Some(PathBuf)` with the path to the .tudu file if found, otherwise `None`.
fn find_tudu_file_from(start_dir: PathBuf) -> Option<PathBuf> {
    let mut current_dir = start_dir;
    loop {
        let config_path = current_dir.join(".tudu");
        if config_path.exists() {
            return fs::canonicalize(config_path).ok();
        }

        if !current_dir.pop() {
            return None;
        }
    }
}

/// Reads the project ID from the .tudu file by searching parent directories.
///
/// The .tudu file is expected to contain a line like:
/// PROJECT_ID=123
///
/// # Returns
///
/// Returns `Some(i32)` with the project ID if found and valid, otherwise `None`.
pub fn get_project_id_from_config() -> Option<i32> {
    let current_dir = env::current_dir().ok()?;
    let config_path = find_tudu_file_from(current_dir)?;

    println!("{}", config_path.to_str().unwrap());
    let file = fs::File::open(config_path).ok()?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(value_str) = line.strip_prefix("PROJECT_ID=") {
                if let Ok(id) = value_str.trim().parse::<i32>() {
                    return Some(id);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    fn run_test_in_temp_dir<F>(test_fn: F)
    where
        F: FnOnce(&TempDir),
    {
        let dir = tempfile::tempdir().unwrap();
        test_fn(&dir);
    }

    #[test]
    fn test_find_in_current_dir() {
        run_test_in_temp_dir(|dir| {
            let tudu_path = dir.path().join(".tudu");
            File::create(&tudu_path).unwrap();
            let found_path = find_tudu_file_from(dir.path().to_path_buf()).unwrap();
            assert_eq!(found_path, fs::canonicalize(tudu_path).unwrap());
        });
    }

    #[test]
    fn test_find_in_parent_dir() {
        run_test_in_temp_dir(|parent_dir| {
            let child_dir = parent_dir.path().join("child");
            fs::create_dir(&child_dir).unwrap();

            let tudu_path = parent_dir.path().join(".tudu");
            File::create(&tudu_path).unwrap();

            let found_path = find_tudu_file_from(child_dir).unwrap();
            assert_eq!(found_path, fs::canonicalize(tudu_path).unwrap());
        });
    }

    #[test]
    fn test_not_found() {
        run_test_in_temp_dir(|dir| {
            assert_eq!(find_tudu_file_from(dir.path().to_path_buf()), None);
        });
    }

    #[test]
    fn test_get_project_id_from_config_found() {
        run_test_in_temp_dir(|dir| {
            let original_dir = env::current_dir().unwrap();
            env::set_current_dir(dir.path()).unwrap();

            let tudu_path = dir.path().join(".tudu");
            let mut file = File::create(&tudu_path).unwrap();
            writeln!(file, "PROJECT_ID=42").unwrap();

            assert_eq!(get_project_id_from_config(), Some(42));

            env::set_current_dir(original_dir).unwrap();
        });
    }
}
