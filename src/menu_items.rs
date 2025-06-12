use regex::Regex;
use std::{error, io};

use crate::{
	context::Context,
	department::{Department, DepartmentId},
	errors::ApplicationError,
	traits::OneLiner,
};

pub trait MenuItem {
	fn menuitem_txt(&self) -> &str;
	fn hotkey(&self) -> Option<&str>;
	fn execute(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>>;
}

/**
 * NameCompany
 **/
pub struct NameCompany {
	menuitem_txt: String,
	hotkey: String,
}

impl NameCompany {
	pub fn new() -> Self {
		Self { menuitem_txt: "Name the company".to_string(), hotkey: "n".to_string() }
	}
}

impl MenuItem for NameCompany {
	fn menuitem_txt(&self) -> &str {
		&self.menuitem_txt
	}

	fn hotkey(&self) -> Option<&str> {
		Some(&self.hotkey)
	}

	fn execute(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		println!("What is the new name of the company?");

		let mut input = String::new();
		io::stdin().read_line(&mut input)?;
		ctx.set_company_name(input.trim().to_string());

		Ok(())
	}
}

/**
 * ListDepartments
 **/
pub struct ListDepartments {
	menuitem_txt: String,
	hotkey: String,
}

impl ListDepartments {
	pub fn new() -> Self {
		Self { menuitem_txt: "List department hierarchy".to_string(), hotkey: "ld".to_string() }
	}
}

impl MenuItem for ListDepartments {
	fn menuitem_txt(&self) -> &str {
		&self.menuitem_txt
	}

	fn hotkey(&self) -> Option<&str> {
		Some(&self.hotkey)
	}

	fn execute(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		println!("{}", ctx.company_name());

		for department in ctx.departments() {
			println!("  └ {}", department.one_liner());
		}

		Ok(())
	}
}

/**
 * CreateDepartment
 **/
pub struct CreateDepartment {
	menuitem_txt: String,
	hotkey: String,
}

impl CreateDepartment {
	pub fn new() -> Self {
		Self { menuitem_txt: "Create a new department".to_string(), hotkey: "cd".to_string() }
	}
}

impl MenuItem for CreateDepartment {
	fn menuitem_txt(&self) -> &str {
		&self.menuitem_txt
	}

	fn hotkey(&self) -> Option<&str> {
		Some(&self.hotkey)
	}

	fn execute(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		println!("What's the name of the new department?");
		let mut name = String::new();
		io::stdin().read_line(&mut name)?;
		let name = name.trim();

		println!(
			"Does this department has a parent department?\n(Press \"Enter\" for none, or enter the department ID)"
		);

		let mut parent_dep = String::new();
		io::stdin().read_line(&mut parent_dep)?;

		let maybe_parent_dep_id = if parent_dep.trim() == "" {
			None
		} else {
			match DepartmentId::try_from(parent_dep.trim())? {
				dep_id if ctx.validate_department_id(&dep_id) => Some(dep_id),
				_ => return Err(Box::new(ApplicationError("Unknown department".to_string()))),
			}
		};

		let new_department = Department::new(ctx.get_next_department_id(), name, maybe_parent_dep_id);
		ctx.insert_department(new_department);

		Ok(())
	}
}

/**
 * PrintContext
 **/
pub struct PrintContext();

impl PrintContext {
	pub fn new() -> Self {
		Self()
	}
}

impl MenuItem for PrintContext {
	fn menuitem_txt(&self) -> &str {
		"Print application context (debug)"
	}

	fn hotkey(&self) -> Option<&str> {
		None
	}

	fn execute(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		println!("{:#?}", ctx);
		Ok(())
	}
}
