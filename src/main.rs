use std::env;
use std::fs::{File, create_dir_all, OpenOptions};
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{Path, MAIN_SEPARATOR};

use glob::glob;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use base64_stream::FromBase64Writer;


#[derive(Debug, Serialize, Deserialize)]
struct HttpHeader {
	name: String,
	value: String
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseContent {
	text: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
	headers: Vec<HttpHeader>,
	content: ResponseContent
}

impl Response {
	fn is_image(&self) -> bool {
		let re = Regex::new(r#"image/(gif|png|jpe?g|webp)"#).unwrap();

		for header in &self.headers {
			if header.name.to_ascii_lowercase() == "content-type" && re.is_match(&header.value) {
				return true;
			}
		}

		return false;
	}
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

#[derive(Debug, Serialize, Deserialize)]
struct Browser {
	version: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Har {
	log: Log
}

fn base64_to_file(path: &String, content: &String) -> std::io::Result<()> {
	let mut path_slices: Vec<&str> = path
		.split(MAIN_SEPARATOR)
		.collect();

	path_slices.pop();
	
	let folder_path = path_slices
		.join(&MAIN_SEPARATOR.to_string());

	create_dir_all(folder_path)?;

	let re = Regex::new(r#"\?.*"#).unwrap();
	let cleaned_path = re.replace_all(&path, "").to_string();

	let file_handler = OpenOptions::new()
		.read(true)
		.write(true)
		.create(true)
		.open(&cleaned_path)?;

	let mut stream_writer = FromBase64Writer::new(file_handler);
	stream_writer.write_all(content.as_bytes()).unwrap();
	stream_writer.flush().unwrap();

	println!("[OK] {}", &cleaned_path);
	Ok(())
}

fn get_dist_filename(url: String, dist_directory: &String) -> String {
	let re = Regex::new("(https?://)").unwrap();
	let result = re.replace_all(&url, "");

	Path::new(dist_directory)
		.join(result.to_string())
		.display()
		.to_string()
}

fn export_har_file(path: String, dist_directory: &String) -> std::io::Result<()> {
	let content = read_file_content(path)?;
	let har: Har = serde_json::from_str(&content)?;
	
	for entry in har.log.entries {
		if entry.response.is_image() {
			let filename = get_dist_filename(entry.request.url, dist_directory);
			match entry.response.content.text {
				Some(content) => {
					base64_to_file(&filename, &content)?;
				},
				None => {
					println!("Could not export content for '{}'", filename);
				}
			};
		}
	}
	
	Ok(())
}

fn read_file_content(path: String) -> std::io::Result<String> {
	println!("export_har_file({})", &path);

	let file = File::open(&path)?;
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();

	buf_reader.read_to_string(&mut contents)?;

	Ok(contents)
}

fn export_har_files(root_folder: String, dist_directory: &String) -> Result<(), std::io::Error> {
	println!("export_har_files({})", &root_folder);

	let pattern = format!("{}{}{}", &root_folder, MAIN_SEPARATOR, "**/*.har.json");

	for file in glob(&pattern).expect("Failed to read glob pattern") {
		match file {
			Ok(path) => {
				export_har_file(path.display().to_string(), dist_directory)?;
			},
			Err(e) => {
				return Err(e.into_error());
			}
		}
	}

	Ok(())
}

fn main() -> std::io::Result<()> {
	let root_folder = env::current_dir()?;
	let data_directory = Path::new(&root_folder)
		.join("resources").display().to_string();
	let dist_directory = Path::new(&root_folder)
		.join("dist").display().to_string();

	export_har_files(data_directory, &dist_directory)?;
	Ok(())
}
