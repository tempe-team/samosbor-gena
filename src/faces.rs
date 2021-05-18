use std::fs;
use rand::seq::SliceRandom;

pub struct Faces {
    males: Vec<Face>,
    females: Vec<Face>,
}

pub struct Face {
    pub path: String,
    pub age: u32
}

impl Faces {
    pub fn get_male(&self) -> &Face {
        self.males.choose(&mut rand::thread_rng()).unwrap()
    }
    pub fn get_female(&self) -> &Face {
        self.females.choose(&mut rand::thread_rng()).unwrap()
    }
}

pub fn get_faces() -> Faces {
    let paths = fs::read_dir("./faces/UTKFace-Sad").unwrap();

    let mut males: Vec<Face> = Vec::new();
    let mut females: Vec<Face> = Vec::new();

    for path in paths {
        let path_path = path.unwrap().file_name();
        let path_str = path_path.to_str().unwrap();
        let segments: Vec<_> = path_str.split("_").take(2).collect();
        if let [age, is_female] = &segments[..] {
            let age: u32 = age.parse().unwrap();
            let is_female: u32 = is_female.parse().unwrap();
            if is_female == 1 {
                females.push(Face{
                    path: String::from(path_str),
                    age: age
                });
            } else {
                males.push(Face{
                    path: String::from(path_str),
                    age: age
                });
            }
        }
    }

    Faces {
        males,
        females
    }
}