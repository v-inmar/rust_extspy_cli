use prettytable::{Cell, Row, Table};
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let mut extensions: HashMap<String, u64> = HashMap::new();

    process_directory(&Path::new(&args[1]), &mut extensions);

    let total: u64 = extensions.values().copied().sum();

    println!("Total bytes: {}", total);

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Extension"),
        Cell::new("Bytes"),
        Cell::new(""),
    ]));

    for (key, value) in &extensions {
        let percentage: f64 = (*value as f64 / total as f64) * 100.0; // get percentage

        let three_dec_round: f64 = (percentage * 1000.0).round() / 1000.0;

        table.add_row(Row::new(vec![
            Cell::new(key),
            Cell::new(&value.to_string()),
            Cell::new(&format!("{} %", three_dec_round)),
        ]));
    }

    table.printstd();
}

fn print_usage() {
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("How To Run")]));

    table.add_row(Row::new(vec![Cell::new(
        "cargo run <directory full path>\ni.e. cargo run /my/fullpath/directory",
    )]));

    table.printstd();
}

fn get_file_ext(path: &Path) -> String {
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

fn get_file_size(path: &Path) -> u64 {
    match fs::metadata(&path) {
        Ok(metadata) => metadata.len(),
        Err(_e) => 0,
    }
}

// function that deals if the path is directory
fn process_directory(path: &Path, ext_map: &mut HashMap<String, u64>) {
    let entries = fs::read_dir(path);
    match entries {
        Ok(children) => {
            for child in children {
                match child {
                    Ok(entry) => {
                        if entry.path().is_file() {
                            let ext_name_and_file_size: (String, u64) = process_file(&entry.path());
                            insert_to_hashmap(
                                ext_map,
                                &ext_name_and_file_size.0,
                                ext_name_and_file_size.1,
                            );
                        } else {
                            //recursive call
                            process_directory(&entry.path(), ext_map);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading an entry: {}", e);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading the directory: {}", e);
        }
    }
}

// function that deals if the path is file
fn process_file(path: &Path) -> (String, u64) {
    // get extension
    let ext_string: String = get_file_ext(&path);

    // get file size
    let size: u64 = get_file_size(&path);

    // return tuple of extension and metadata
    return (ext_string, size);
}

// add value to hashmap
fn insert_to_hashmap(ext_map: &mut HashMap<String, u64>, ext_name: &String, file_size: u64) {
    if let Some(size) = ext_map.get_mut(ext_name) {
        *size += file_size;
    } else {
        ext_map.insert(ext_name.to_string(), file_size);
    }
}
