use std::{error, io};

use crate::{
	context::Context,
	department::{Department, DepartmentId},
	errors::ApplicationError,
	traits::OneLiner,
};

pub trait MenuItem {
	fn menuitem_txt(&self) -> &str;
	fn shortcut(&self) -> Option<&str>;
	fn execute(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>>;
}

/**
 * NameCompany
 **/
pub struct NameCompany {
	menuitem_txt: String,
	shortcut: String,
}

impl NameCompany {
	pub fn new() -> Self {
		Self { menuitem_txt: "Name the company".to_string(), shortcut: "n".to_string() }
	}
}

impl MenuItem for NameCompany {
	fn menuitem_txt(&self) -> &str {
		&self.menuitem_txt
	}

	fn shortcut(&self) -> Option<&str> {
		Some(&self.shortcut)
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
	shortcut: String,
}

impl ListDepartments {
	pub fn new() -> Self {
		Self { menuitem_txt: "List department hierarchy".to_string(), shortcut: "ld".to_string() }
	}

	fn department_and_children_one_liners(&self, ctx: &Context, dep: &Department, level: u32) -> String {
		let mut result: String = format!("{}L {}\n", "  ".repeat(level as usize), dep.one_liner());

		let dep_str = ctx
			.departments()
			.iter()
			.filter(|d| *d.parent() == Some(*dep.id()))
			.map(|d| self.department_and_children_one_liners(ctx, d, level + 1))
			.fold(String::new(), |acc, line| acc + &line); // this is the way to concatenate two strings with a return value, if we don't want to use format!() macro call.

		result.push_str(&dep_str);
		result
	}
}

impl MenuItem for ListDepartments {
	fn menuitem_txt(&self) -> &str {
		&self.menuitem_txt
	}

	fn shortcut(&self) -> Option<&str> {
		Some(&self.shortcut)
	}

	fn execute(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		let mut result = format!("{}\n", ctx.company_name());

		let dep_str = ctx
			.departments()
			.iter()
			.filter(|dep| dep.parent().is_none())
			.map(|dep| self.department_and_children_one_liners(ctx, dep, 0))
			.fold(String::new(), |acc, line| acc + &line); // this is the way to concatenate two strings with a return value, if we don't want to use format!() macro call.

		result.push_str(&dep_str);

		println!("{}", result);
		Ok(())
	}
}

/**
 * CreateDepartment
 **/
pub struct CreateDepartment {
	menuitem_txt: String,
	shortcut: String,
}

impl CreateDepartment {
	pub fn new() -> Self {
		Self { menuitem_txt: "Create a new department".to_string(), shortcut: "cd".to_string() }
	}
}

impl MenuItem for CreateDepartment {
	fn menuitem_txt(&self) -> &str {
		&self.menuitem_txt
	}

	fn shortcut(&self) -> Option<&str> {
		Some(&self.shortcut)
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
 * ShowDepartment
 **/
pub struct ShowDepartment();

impl MenuItem for ShowDepartment {
	fn menuitem_txt(&self) -> &str {
		"Show department information"
	}

	fn shortcut(&self) -> Option<&str> {
		Some("rd")
	}

	fn execute(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		let max_input = Into::<u32>::into(*ctx.next_department_id()) - 1;
		println!("Which department do you want to show? (0 - {})", max_input);

		let mut input = String::new();
		io::stdin().read_line(&mut input)?;

		if let Some(dep_info) = ctx.department_info(&DepartmentId(input.trim().parse::<u32>()?)) {
			println!("{}", dep_info);
		} else {
			return Err(Box::new(ApplicationError("Unknown department".to_string())));
		}

		Ok(())
	}
}

/**
 * PrintContext
 **/
pub struct PrintContext();

impl MenuItem for PrintContext {
	fn menuitem_txt(&self) -> &str {
		"Print application context (debug)"
	}

	fn shortcut(&self) -> Option<&str> {
		None
	}

	fn execute(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		println!("{:#?}", ctx);
		Ok(())
	}
}

/**
 * Save Context
 **/
pub struct SaveContext();

impl MenuItem for SaveContext {
	fn menuitem_txt(&self) -> &str {
		"Save to a file"
	}

	fn shortcut(&self) -> Option<&str> {
		Some("s")
	}

	fn execute(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		println!("{:#?}", ctx);
		Ok(())
	}
}

/**
 * Load Context
 **/
pub struct LoadContext();

impl MenuItem for LoadContext {
	fn menuitem_txt(&self) -> &str {
		"Load from a file"
	}

	fn shortcut(&self) -> Option<&str> {
		Some("l")
	}

	fn execute(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		println!("{:#?}", ctx);
		Ok(())
	}
}

/**
 * Quit
 **/
pub struct Quit();

impl MenuItem for Quit {
	fn menuitem_txt(&self) -> &str {
		"Quit"
	}

	fn shortcut(&self) -> Option<&str> {
		Some("q")
	}

	fn execute(&self, _: &mut Context) -> Result<(), Box<dyn error::Error>> {
		std::process::exit(0);
	}
}
