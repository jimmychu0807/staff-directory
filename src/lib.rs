use regex::Regex;
use std::{
	boxed::Box,
	error,
	io::{self, Write},
};

pub mod department;
pub mod staff;
pub mod traits;

use crate::department::{CreateDepartment, ListDepartments};
use crate::traits::MenuItem;

fn display_menu(menu_items: &[Box<dyn MenuItem>]) -> Result<(), Box<dyn error::Error>> {
	println!("What do you want to do?");

	for (idx, item) in menu_items.iter().enumerate() {
		println!("{}. {:25}         [{}]", idx + 1, item.menuitem_txt(), item.hotkey());
	}

	print!("? ");
	let _ = io::stdout().flush();

	Ok(())
}

pub fn run() -> Result<(), Box<dyn error::Error>> {
	let menu_items: Vec<Box<dyn MenuItem>> =
		vec![Box::new(ListDepartments::new()), Box::new(CreateDepartment::new())];
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
