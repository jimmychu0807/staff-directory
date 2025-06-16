use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
	let cli = directory_rs::parse();
	directory_rs::run(Some(cli))
}
