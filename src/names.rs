use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rand::seq::SliceRandom;

pub struct Names {
    pub male_names: Vec<String>,
    pub male_surnames: Vec<String>,
    pub female_names: Vec<String>,
    pub female_surnames: Vec<String>,
}

impl Names {
    pub fn get_male(&self) -> String {
        format!(
            "{} {}",
            self.male_names.choose(&mut rand::thread_rng()).unwrap(),
            self.male_surnames.choose(&mut rand::thread_rng()).unwrap(),
        )
    }
    pub fn get_female(&self) -> String {
        format!(
            "{} {}",
            self.female_names.choose(&mut rand::thread_rng()).unwrap(),
            self.female_surnames.choose(&mut rand::thread_rng()).unwrap(),
        )
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_names() -> Names {
    let mut male_names: Vec<String> = Vec::new();
    let mut male_surnames: Vec<String> = Vec::new();
    let mut female_names: Vec<String> = Vec::new();
    let mut female_surnames: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./names/FIO_Men_Women/baseMenName.txt") {
        for line in lines {
            if let Ok(text) = line {
                male_names.push(text);
            }
        }
    }
    if let Ok(lines) = read_lines("./names/FIO_Men_Women/baseMenFam.txt") {
        for line in lines {
            if let Ok(text) = line {
                male_surnames.push(text);
            }
        }
    }
    if let Ok(lines) = read_lines("./names/FIO_Men_Women/baseWomenName.txt") {
        for line in lines {
            if let Ok(text) = line {
                female_names.push(text);
            }
        }
    }
    if let Ok(lines) = read_lines("./names/FIO_Men_Women/baseWomenFam.txt") {
        for line in lines {
            if let Ok(text) = line {
                female_surnames.push(text);
            }
        }
    }
    Names {
        male_names,
        male_surnames,
        female_names,
        female_surnames
    }
}
