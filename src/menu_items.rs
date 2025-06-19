use std::{error, fs, io, path::Path};

use crate::{
	context::Context,
	department::{Department, DepartmentId, DepartmentInfo},
	errors::ApplicationError,
	traits::OneLiner,
};

pub trait MenuItem {
	fn menuitem_txt(&self) -> &str;
	fn shortcut(&self) -> Option<&str>;
	fn execute_interactive(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>>;
	fn execute<'a>(
		&self,
		ctx: &'a mut Context,
		input: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>>;
}

#[derive(Debug)]
pub enum MenuItemInput {
	String(String),
	DepartmentParams(String, Option<String>),
	None,
}

#[derive(Debug)]
pub enum MenuItemOutput<'a> {
	String(String),
	DepartmentInfo(DepartmentInfo<'a>),
	None,
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

	fn execute_interactive(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		println!("What is the new name of the company?");

		let mut input = String::new();
		io::stdin().read_line(&mut input)?;

		let _ = self.execute(ctx, MenuItemInput::String(input.trim().to_string()))?;
		Ok(())
	}

	fn execute<'a>(
		&self,
		ctx: &'a mut Context,
		input: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>> {
		let MenuItemInput::String(input) = input else {
			Err(Box::new(ApplicationError(format!("Unrecognized input: {:?}", input))))?
		};

		ctx.set_company_name(input.to_string());

		Ok(MenuItemOutput::None)
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

	fn department_and_children_one_liners(ctx: &Context, dep: &Department, level: u32) -> String {
		let mut result: String = format!("{}L {}\n", "  ".repeat(level as usize), dep.one_liner());

		let dep_str = ctx
			.departments()
			.iter()
			.filter(|d| *d.parent() == Some(*dep.id()))
			.map(|d| Self::department_and_children_one_liners(ctx, d, level + 1))
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

	fn execute_interactive(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		let MenuItemOutput::String(result) = self.execute(ctx, MenuItemInput::None)? else {
			Err(Box::new(ApplicationError("Unrecognized output".to_string())))?
		};

		println!("{result}");
		Ok(())
	}

	fn execute<'a>(
		&self,
		ctx: &'a mut Context,
		_input: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>> {
		let mut result = format!("{}\n", ctx.company_name());

		let dep_str = ctx
			.departments()
			.iter()
			.filter(|dep| dep.parent().is_none())
			.map(|dep| Self::department_and_children_one_liners(ctx, dep, 0))
			.fold(String::new(), |acc, line| acc + &line); // this is the way to concatenate two strings with a return value, if we don't want to use format!() macro call.

		result.push_str(&dep_str);

		Ok(MenuItemOutput::String(result))
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

	fn execute_interactive(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		println!("What's the name of the new department?");
		let mut name = String::new();
		io::stdin().read_line(&mut name)?;
		let name = name.trim();

		println!(
			"Does this department has a parent department?\n(Press \"Enter\" for none, or enter the department ID)"
		);
		let mut parent_dep = String::new();
		io::stdin().read_line(&mut parent_dep)?;
		let parent_dep = parent_dep.trim();
		let parent_dep = if parent_dep.len() > 0 { Some(parent_dep.to_string()) } else { None };

		self.execute(ctx, MenuItemInput::DepartmentParams(name.to_string(), parent_dep)).map(|_| ())
	}

	fn execute<'a>(
		&self,
		ctx: &'a mut Context,
		input: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>> {
		let MenuItemInput::DepartmentParams(name, parent_dep_id) = input else {
			Err(Box::new(ApplicationError("Unrecognized input".to_string())))?
		};

		let parent_dep_id = match parent_dep_id {
			Some(parent_dep_id) => {
				let parent_dep_id = <DepartmentId as TryFrom<&str>>::try_from(&parent_dep_id)?;

				if ctx.validate_department_id(&parent_dep_id) {
					Some(parent_dep_id)
				} else {
					Err(Box::new(ApplicationError("Unknown department".to_string())))?
				}
			}
			None => None,
		};

		let new_department = Department::new(ctx.get_next_department_id(), &name, parent_dep_id);
		ctx.insert_department(new_department);

		Ok(MenuItemOutput::None)
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

	fn execute_interactive(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		let max_input = Into::<u32>::into(*ctx.next_department_id()) - 1;
		println!("Which department do you want to show? (0 - {})", max_input);

		let mut input = String::new();
		io::stdin().read_line(&mut input)?;

		let MenuItemOutput::DepartmentInfo(dep_info) =
			self.execute(ctx, MenuItemInput::String(input.trim().to_string()))?
		else {
			Err(Box::new(ApplicationError("Unrecognized output".to_string())))?
		};

		println!("{dep_info}");
		Ok(())
	}

	fn execute<'a>(
		&self,
		ctx: &'a mut Context,
		input: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>> {
		let MenuItemInput::String(dep_id) = input else {
			Err(Box::new(ApplicationError("Unrecognized input".to_string())))?
		};

		let Some(dep_info) = ctx.department_info(&DepartmentId(dep_id.parse::<u32>()?)) else {
			Err(Box::new(ApplicationError("Invalid department ID".to_string())))?
		};

		Ok(MenuItemOutput::DepartmentInfo(dep_info))
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

	fn execute_interactive(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		let MenuItemOutput::String(output) = self.execute(ctx, MenuItemInput::None)? else {
			Err(Box::new(ApplicationError("Unrecognized output".to_string())))?
		};

		println!("{output}");
		Ok(())
	}

	fn execute<'a>(
		&self,
		ctx: &'a mut Context,
		_input: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>> {
		Ok(MenuItemOutput::String(format!("{:#?}", ctx)))
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

	fn execute_interactive(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		println!("Which file path to save to?");
		let mut filepath = String::new();
		io::stdin().read_line(&mut filepath)?;

		self.execute(ctx, MenuItemInput::String(filepath.trim().to_string())).map(|_| ())
	}

	fn execute<'a>(
		&self,
		ctx: &'a mut Context,
		input: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>> {
		let MenuItemInput::String(filepath) = input else {
			Err(Box::new(ApplicationError(format!("Unrecognized input: {:?}", input))))?
		};
		let path = Path::new(&filepath);

		let serialized = serde_json::to_string(ctx)?;
		fs::write(path, serialized).map(|_| MenuItemOutput::None).map_err(|e| e.into())
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

	fn execute_interactive(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		println!("Which file path to load from?");

		let mut filepath = String::new();
		io::stdin().read_line(&mut filepath)?;

		self.execute(ctx, MenuItemInput::String(filepath.trim().to_string())).map(|_| ())
	}

	fn execute<'a>(
		&self,
		ctx: &'a mut Context,
		input: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>> {
		let MenuItemInput::String(filepath) = input else {
			Err(Box::new(ApplicationError(format!("Unrecognized input: {:?}", input))))?
		};

		let data_filepath = Path::new(&filepath);
		let content = fs::read_to_string(data_filepath)?;

		*ctx = serde_json::from_str::<Context>(&content)?;

		Ok(MenuItemOutput::None)
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

	fn execute_interactive(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		self.execute(ctx, MenuItemInput::None).map(|_| ())
	}

	fn execute<'a>(
		&self,
		_: &'a mut Context,
		_: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>> {
		std::process::exit(0);
	}
}
