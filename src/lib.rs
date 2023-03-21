use structopt::StructOpt;
use std::{path::Path, fs, fs::File, io::{BufReader, BufRead}};

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(short, long, requires="word")]
    pub path: Option<String>,

    #[structopt(short, long, requires="path")]
    pub word: Option<String>,

    #[structopt(short="t", long)]
    pub filetype: Option<String>
}

#[derive(Debug)]
pub struct OutResult {
    pub file_name: String,
    pub word: String,
    pub line: i32,
}

pub fn find_in_file(file_name: &Path, search_word: &String) -> Option<Vec<OutResult>>{
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("File '{}' does not exists", file_name.to_string_lossy().into_owned());
            return None
        },
    };

    let mut results = Vec::new();
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            if line.contains(search_word) {
                results.push(OutResult {
                    file_name: file_name.to_string_lossy().into_owned(),
                    word: search_word.clone(),
                    line: i as i32,
                });
            }
        }
    }

    Some(results)
}

pub fn find_in_dir(path: &Path, word: &String, file_ext: &Option<String>) -> Option<Vec<Vec<OutResult>>>{
    if !path.exists() {
        println!("Path does not exists");
        return None;
    }
    let mut results = Vec::new();

    if !path.is_dir() {
        match find_in_file(&path, &word) {
            Some(curr_res) => {
                results.push(curr_res);
            },
            None => {return None;}
        }
    }

    for entry in fs::read_dir(path).unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if !path.is_file() {continue;}

            let curr_file_ext = path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");

            if let Some(ref file_ext) = file_ext {
                if curr_file_ext != file_ext {
                    continue;
                }
            }

            match find_in_file(&path, &word) {
                Some(cur_res) => {
                    results.push(cur_res);
                },
                None => {},
            }
        }
    }
    Some(results)
}