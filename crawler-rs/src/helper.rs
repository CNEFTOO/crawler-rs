use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub fn get_md5(input: &str) -> String {
    let digest = md5::compute(input);
    format!("{:x}", digest)
}

pub fn convert_headers(h: HashMap<String, String>) -> HashMap<String, String> {
    h.into_iter().map(|(k, v)| (k, v)).collect()
}

pub fn write_file(filename: &str, data: &[u8]) {
    match OpenOptions::new().write(true).create(true).open(filename) {
        Ok(mut file) => {
            if let Err(err) = file.write_all(data) {
                eprintln!("Failed to write file: {}", err);
            }
        }
        Err(e) => {
            eprintln!("Error opening file : {}", e);
        }
    }
}

pub fn read_file(filepath: &str) -> Vec<String> {
    let mut lines = Vec::new();
    if let Ok(file) = File::open(filepath) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                lines.push(line);
            }
        }
    } else {
        eprintln!("Error opening file: {}", filepath);
    }
    lines
}

pub fn string_slice_contain(data: &[String], item: &str) -> bool {
    data.iter().any(|x| x == item)
}

pub fn map_string_format(data: &HashMap<String, String>) -> String {
    data.iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join(",")
}