use clap::Parser;
use regex::Regex;
use std::{
	boxed::Box,
	error, fs,
	io::{self, Write},
	path::PathBuf,
};

pub mod context;
pub mod department;
pub mod errors;
pub mod menu_items;
pub mod staff;
pub mod traits;

#[cfg(test)]
mod tests;

use crate::{
	context::Context,
	menu_items::{
		CreateDepartment, CreateStaff, ListDepartments, ListStaff, LoadContext, MenuItem, NameCompany,
		PrintContext, Quit, SaveContext, ShowDepartment,
	},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
	data_file: Option<PathBuf>,

	#[arg(short, long, default_value_t = false)]
	debug: bool,
}

pub fn run(cli: Option<Cli>) -> Result<(), Box<dyn error::Error>> {
	// load the context if specified
	let mut ctx = if cli.is_none() || cli.as_ref().unwrap().data_file.is_none() {
		Context::new()
	} else {
		let data_filepath = cli.as_ref().unwrap().data_file.as_ref().unwrap();
		let content = fs::read_to_string(data_filepath)?;
		serde_json::from_str::<Context>(&content)?
	};

	let debug = cli.as_ref().unwrap().debug;

	let mut menu_items: Vec<Box<dyn MenuItem>> = vec![
		Box::new(NameCompany::new()),
		Box::new(ListDepartments::new()),
		Box::new(CreateDepartment::new()),
		Box::new(ShowDepartment()),
		Box::new(ListStaff()),
		Box::new(CreateStaff()),
		Box::new(SaveContext()),
		Box::new(LoadContext()),
	];
	if debug {
		menu_items.push(Box::new(PrintContext()));
	}
	menu_items.push(Box::new(Quit()));

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
					Some(choice) if choice < menu_items.len() => {
						menu_items[choice].execute_interactive(&mut ctx)
					}
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
			t if !t.is_empty() => {
				if let Some(mi) = get_menu_item_from_shortcut(&menu_items, t) {
					let _ = mi.execute_interactive(&mut ctx);
				} else {
					println!("Invalid shortcut");
				}
			}
			_ => continue,
		};
	}
}

pub fn parse() -> Cli {
	Cli::parse()
}

/**
 * Un-export / internal helper methods below
 **/
fn display_menu(menu_items: &[Box<dyn MenuItem>]) -> Result<(), Box<dyn error::Error>> {
	println!("What do you want to do?");

	for (idx, item) in menu_items.iter().enumerate() {
		println!(
			"{}. {:40}{}",
			idx + 1,
			item.menuitem_txt(),
			if let Some(ks) = item.shortcut() { format!("[{ks}]") } else { "".to_string() }
		);
	}

	print!("? ");
	let _ = io::stdout().flush();

	Ok(())
}

fn get_menu_item_from_shortcut<'a>(
	menu_items: &'a [Box<dyn MenuItem>],
	shortcut: &'a str,
) -> Option<&'a dyn MenuItem> {
	if shortcut.is_empty() {
		return None;
	}

	menu_items
		.iter()
		.filter(|mi| mi.shortcut().map_or("", |sc| sc) == shortcut)
		.collect::<Vec<_>>()
		.first()
		.map(|bv| &***bv)
}
