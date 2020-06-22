use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use glob::glob;
use serde_json::{Result, Value};

fn export_har_file(path: String) {
	let content = match read_file_content(path) {
		Ok(contents) => contents,
		Err(e) => e.to_string()
	};

	// println!("{}", content);
	let data = match serde_json::from_str(&content).expect("Failed to read JSON data") {
		Ok(content) => content
	};

	println!("{}", data);
}

fn read_file_content(path: String) -> Result<String, std::io::Error> {
	println!("export_har_file({})", &path);

	let file = match File::open(&path) {
		Ok(file) => file,
		Err(e) => return Err(e)
	};

	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();

	match buf_reader.read_to_string(&mut contents) {
		Ok(_) => Ok(contents),
		Err(e) => Err(e),
	}
}

fn export_har_files(root_folder: String) {
	println!("export_har_files({})", &root_folder);

	let pattern = format!("{}/{}", &root_folder, "**/*.har.json");

	for file in glob(&pattern).expect("Failed to read glob pattern") {
	 match file {
		 Ok(path) => export_har_file(path.display().to_string()),
		 Err(e) => println!("{:?}", e)
	 }
	}
}

fn main() -> std::io::Result<()> {
	let root_folder = env::current_dir()?;
	let data_directory = Path::new(&root_folder)
		.join("resources").display().to_string();

	export_har_files(data_directory);
	Ok(())
}
