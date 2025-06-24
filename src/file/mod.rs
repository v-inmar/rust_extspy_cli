use std::path::Path;
use std::fs;

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

