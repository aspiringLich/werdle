use std::borrow::Borrow;
use std::fmt::Debug;
use std::fs::File;
use std::fs::read_to_string;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write, Lines};
use std::path::Path;

// load a file into a vector
pub fn load_into_vec(path: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for line in path.lines() {
        out.push(line.to_string());
    }
    out
}


// parse raw csv file and organize by length
pub fn _parse_raw_data_by_len() {
    let raw_data: &Path = Path::new("data/unigram_freq.csv");
    let raw_length: String = "data/raw_length/".to_string();
    let file = read_to_string(raw_data).unwrap();
    let mut itr = 0;
    // over all the lines in the file (except the first)
    for line in file.lines().skip(1) {
        print!("\r{} read out of however many", itr);
        itr += 1;
        let split: Vec<&str> = line.split(",").collect();
        let path = &format!("{}{}.txt", &raw_length, split[0].len()); // get the length of the word
        let mut file = std::fs::OpenOptions::new()  // do some things with the file
            .append(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();

        // if Path::new(path).exists() { file.open(path).unwrap(); }
                            //    else { file.create(path).unwrap(); }
        file.write_all(format!("{}\n", line).as_bytes());
    }
    println!();
}

// alphabetizes a file from p_in to p_out
pub fn _alphabetize(p_in: &Path, p_out: &Path) {
    let in_file = read_to_string(p_in).unwrap();
    let mut words: [Vec<&str>; 26] = Default::default();

    // sort it all out
    for (i, line) in in_file.lines().enumerate() {
        print!("\r{i} read out of however many");
        let split: Vec<&str> = line.split(',').collect();
        let word: &str = split[0];
        let index = word.chars().next().unwrap() as usize - 'a' as usize;
        words[index].push(word);
    }

    let mut out_file = std::fs::OpenOptions::new() 
        .append(true)
        .write(true)
        .create(true)
        .open(p_out)
        .unwrap();

    // actually sort it and dump it into the output file
    for mut item in words {
        item.sort();
        for word in item {
            out_file.write_all(format!("{}\n", word).as_bytes());
        }
    }
}