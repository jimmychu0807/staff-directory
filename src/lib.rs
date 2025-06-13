use regex::Regex;
use std::{
	boxed::Box,
	error,
	io::{self, Write},
};

pub mod context;
pub mod department;
pub mod errors;
pub mod menu_items;
pub mod staff;
pub mod traits;

use crate::{
	context::Context,
	menu_items::{CreateDepartment, ListDepartments, MenuItem, NameCompany, PrintContext, ShowDepartment},
};

fn display_menu(menu_items: &[Box<dyn MenuItem>]) -> Result<(), Box<dyn error::Error>> {
	println!("What do you want to do?");

	for (idx, item) in menu_items.iter().enumerate() {
		println!(
			"{}. {:40}{}",
			idx + 1,
			item.menuitem_txt(),
			match item.hotkey() {
				Some(ks) => format!("[{ks}]"),
				None => "".to_string(),
			}
		);
	}

	print!("? ");
	let _ = io::stdout().flush();

	Ok(())
}

pub fn run() -> Result<(), Box<dyn error::Error>> {
	let mut ctx = Context::new();

	let menu_items: Vec<Box<dyn MenuItem>> = vec![
		Box::new(NameCompany::new()),
		Box::new(ListDepartments::new()),
		Box::new(CreateDepartment::new()),
		Box::new(ShowDepartment::new()),
		#[cfg(debug_assertions)]
		Box::new(PrintContext::new()),
	];
	let re_digits = Regex::new(r"\d+$")?;

	loop {
		display_menu(&menu_items)?;

		let mut input = String::new();
		io::stdin().read_line(&mut input)?;
		let input_str = input.trim();

		match input_str {
			t if re_digits.is_match(t) => {
				// Have to minus 1 from user input, as internally it is zero-offset.
				let result = match t.parse::<usize>()?.checked_sub(1) {
					Some(choice) if choice < menu_items.len() => menu_items[choice].execute(&mut ctx),
					_ => {
						println!("Invalid choice");
						Ok(())
					}
				};

				// Print error out, if any
				if let Err(err) = result {
					println!("{}", err);
				}

				// Append one more newline at the end
				println!();
			}
			"q" => break,
			_ => continue,
		};
	}

	Ok(())
}
