use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use glob::glob;

fn export_har_file(path: String) -> Result<String, std::io::Error> {
	println!("export_har_file({})", &path);

	let mut file = match File::open(&path) {
		Ok(file) => file,
		Err(e) => return Err(e)
	};

	let mut buf_reader = BufReader::new(file);

	let mut contents = String::new();

	match buf_reader.read_to_string(&mut content) {
		Ok(_) => s,
		Err(e) => Err(e),
	}
	// buf_reader.read_to_string(&mut contents);
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
