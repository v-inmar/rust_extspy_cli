use std::fs;
use std::path::Path;

pub fn get_file_ext(path: &Path) -> String {
    if let Some(ext) = path.extension() {
        return ext // returns Option<&OsStr> (to handle invalid utf8)
            .to_string_lossy() // produces Cow<str> (clone on write)
            .into_owned(); // guarantees owned string
    } else {
        // path.extensions returns None for files such as .gitignore because they are classified as having no extensions
        // so we check them
        if let Some(name) = path.file_name() {
            let lossy_name_str = name.to_string_lossy();
            if lossy_name_str.starts_with(".") && lossy_name_str.len() > 1 {
                return lossy_name_str.into_owned().split_off(1);
            }
        }

        // there are cases where there are files doesnt have extensions
        // those are the unknowns
        // i.e. /target/debug/.fingerprint/anstyle-39cb1377a2abc41e/dep-lib-anstyle
        // something to work on in the future
        return String::from("unknown");
    }
}

pub fn get_file_size(path: &Path) -> u64 {
    match fs::metadata(&path) {
        Ok(metadata) => metadata.len(),
        Err(_e) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    fn temp_test_file(name: &str, content: Option<&str>) -> std::path::PathBuf {
        let temp_dir = env::temp_dir();
        let file_path = temp_dir.join(name);
        if let Some(content) = content {
            let mut file = File::create(&file_path).expect("Unable to create file");
            file.write_all(content.as_bytes())
                .expect("Unable to write to file");
        }
        file_path
    }

    #[test]
    fn test_get_file_ext_with_valid_extension() {
        let path = Path::new("/path/to/file.txt");
        assert_eq!(get_file_ext(&path), "txt");
    }

    #[test]
    fn test_get_file_ext_no_extension() {
        let path = Path::new("/path/to/README");
        assert_eq!(get_file_ext(&path), "unknown");
    }

    #[test]
    fn test_get_file_ext_hidden_file() {
        let path = Path::new("/path/to/.gitignore");
        assert_eq!(get_file_ext(&path), "gitignore");
    }

    #[test]
    fn test_get_file_ext_directory_path() {
        let path = Path::new("/path/to/some/directory/");
        assert_eq!(get_file_ext(&path), "unknown");
    }

    #[test]
    fn test_get_file_ext_with_multiple_extensions() {
        let path = Path::new("/path/to/archive.tar.gz");
        assert_eq!(get_file_ext(&path), "gz");
    }

    #[test]
    fn test_get_file_size_valid_file() {
        let path = temp_test_file("test_file.txt", Some("This is a test file"));
        assert_eq!(get_file_size(&path), 19); // File length is 19 bytes
    }

    #[test]
    fn test_get_file_size_non_existent_file() {
        let path = Path::new("/path/to/non_existent_file.txt");
        assert_eq!(get_file_size(&path), 0);
    }

    #[test]
    fn test_get_file_size_directory() {
        let path = Path::new("/path/to/some/directory/");
        assert_eq!(get_file_size(&path), 0);
    }
}
