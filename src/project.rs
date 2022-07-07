use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Project {
    main_file: File,
    project_file: File,
}

impl Project {
    fn new(proj_name: String) -> Project {
        let mut file = File::open("foo.txt").unwrap();
        let mut json_data = String::new();

        file.read_to_string(&mut json_data);

        let project = json::parse(&json_data).unwrap();

    }
}