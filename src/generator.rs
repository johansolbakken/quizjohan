use std::fs::File;
use std::io::prelude::*;

pub fn create_directory(dir: &String) {
    std::fs::create_dir_all(dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });
}

pub fn create_style_sheet(template_file: &String, dir: &String) {
    let mut file = File::create(dir).unwrap();
    let mut template = File::open(template_file).unwrap();
    let mut contents = String::new();
    template.read_to_string(&mut contents).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

pub fn create_html_file(template_file: &String, dir: &String) {
    let mut file = File::create(dir).unwrap();
    let mut template = File::open(template_file).unwrap();
    let mut contents = String::new();
    template.read_to_string(&mut contents).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
