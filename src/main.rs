use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
	let cli = staff_directory::parse();
	staff_directory::run(Some(cli))
}
