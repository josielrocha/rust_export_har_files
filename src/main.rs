use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use glob::glob;
use serde_json;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct HttpHeader {
	name: String,
	value: String
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseContent {
	text: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
	headers: HttpHeader,
	content: ResponseContent
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
	url: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Resource {
	request: Request,
	response: Response
}

#[derive(Debug, Serialize, Deserialize)]
struct Log {
	entries: Vec<Resource>
}

fn export_har_file(path: String) -> std::io::Result<()> {
	let content = read_file_content(path)?;
	let log: Log = serde_json::from_str(&content)?;
	
	for entry in log.entries {
		println!("{}", 1)
	}
	
	Ok(())
}

fn read_file_content(path: String) -> std::io::Result<String> {
	println!("export_har_file({})", &path);

	let file = File::open(&path)?;
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();

	println!("{}", "JOSIEL");

	match buf_reader.read_to_string(&mut contents) {
		Ok(_) => Ok(contents),
		Err(e) => Err(e)
	}
}

fn export_har_files(root_folder: String) -> std::io::Result<()> {
	println!("export_har_files({})", &root_folder);

	let pattern = format!("{}/{}", &root_folder, "**/*.har.json");

	for file in glob(&pattern).expect("Failed to read glob pattern") {
		match file {
			Ok(path) => export_har_file(path.display().to_string()),
			Err(e) => println!("{}", e.to_string())
		}
	}

	Ok(())
}

fn main() -> std::io::Result<()> {
	let root_folder = env::current_dir()?;
	let data_directory = Path::new(&root_folder)
		.join("resources").display().to_string();

	export_har_files(data_directory);
	Ok(())
}
