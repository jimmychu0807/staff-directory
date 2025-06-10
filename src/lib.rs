use std::{
	error, io::{ self, Write }
};
use regex::Regex;

pub mod department;
pub mod traits;
pub mod staff;

use crate::department::ListDepartments;
use crate::traits::MenuItem;

fn display_menu<T: MenuItem>(
	menu_items: &[T]
) -> Result<(), Box<dyn error::Error>> {
	println!("What do you want to do?");

	for (idx, item) in menu_items.iter().enumerate() {
		println!("{}. {}", idx + 1, item.menuitem_txt());
	}

	print!("? ");
	let _ = io::stdout().flush();

	Ok(())
}

pub fn run() -> Result<(), Box<dyn error::Error>> {
	let menu_items = vec![ListDepartments::new()];
	let re_digits = Regex::new(r"\d+$")?;

	loop {
		display_menu(&menu_items)?;

		let mut input = String::new();
		io::stdin().read_line(&mut input)?;
		let input_str: &str = &input.trim();

		match input_str {
			t if re_digits.is_match(t) => {
				// Have to minus 1 from user input, as internally it is zero-offset.
				match t.parse::<usize>()?.checked_sub(1) {
					Some(choice) if choice < menu_items.len() => menu_items[choice].execute(),
					_ => println!("Invalid choice"),
				}
			}
			"q" => break,
			_ => continue,
		};
	}

	Ok(())
}
