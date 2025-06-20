use chrono::NaiveDate;
use std::{
	error, fs,
	io::{self, Write},
	path::Path,
};

use crate::{
	context::Context,
	department::{Department, DepartmentBuilder, DepartmentId, DepartmentInfo},
	errors::ApplicationError,
	staff::{Gender, Staff, StaffBuilder},
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

pub enum MenuItemInput {
	String(String),
	DepartmentBuilder(DepartmentBuilder),
	StaffBuilder(StaffBuilder),
	None,
}

pub enum MenuItemOutput<'a> {
	String(String),
	Department(&'a Department),
	Staff(&'a Staff),
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
			Err(Box::new(ApplicationError("Unrecognized input".to_string())))?
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

		result.push_str(dep_str.trim());

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
		let name = name.trim().to_string();

		println!(
			"Does this department has a parent department?\n(Press \"Enter\" for none, or enter the department ID)"
		);
		let mut parent = String::new();
		io::stdin().read_line(&mut parent)?;
		let parent = parent.trim();
		let parent =
			if !parent.is_empty() { Some(<DepartmentId as TryFrom<&str>>::try_from(parent)?) } else { None };

		self.execute(ctx, MenuItemInput::DepartmentBuilder(DepartmentBuilder::new(name, parent))).map(|_| ())
	}

	fn execute<'a>(
		&self,
		ctx: &'a mut Context,
		input: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>> {
		let MenuItemInput::DepartmentBuilder(builder) = input else {
			Err(Box::new(ApplicationError("Unrecognized input".to_string())))?
		};

		let dep = ctx.insert_department(builder)?;
		Ok(MenuItemOutput::Department(dep))
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
		let next_dep_id = <DepartmentId as Into<u32>>::into(*ctx.next_department_id());
		if next_dep_id == 0 {
			println!("No department exists yet.");
			return Ok(());
		}

		// let max_input = Into::<u32>::into(*ctx.next_department_id()) - 1;
		let max_input = next_dep_id - 1;
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

pub struct ListStaff();

impl MenuItem for ListStaff {
	fn menuitem_txt(&self) -> &str {
		"List all staff"
	}

	fn shortcut(&self) -> Option<&str> {
		Some("ls")
	}

	fn execute_interactive(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		Ok(())
	}

	fn execute<'a>(
		&self,
		ctx: &'a mut Context,
		input: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>> {
		Ok(MenuItemOutput::None)
	}
}

pub struct CreateStaff();

impl MenuItem for CreateStaff {
	fn menuitem_txt(&self) -> &str {
		"Create a new staff"
	}

	fn shortcut(&self) -> Option<&str> {
		Some("cs")
	}

	fn execute_interactive(&self, ctx: &mut Context) -> Result<(), Box<dyn error::Error>> {
		let mut first_name = String::new();
		loop {
			print!("First name: ");
			io::stdout().flush().unwrap();
			io::stdin().read_line(&mut first_name)?;
			first_name = first_name.trim().to_string();
			if !first_name.is_empty() {
				break;
			}
			first_name.clear();
		}

		let mut last_name = String::new();
		loop {
			print!("Last name: ");
			io::stdout().flush().unwrap();
			io::stdin().read_line(&mut last_name)?;
			last_name = last_name.trim().to_string();
			if !last_name.is_empty() {
				break;
			}
			last_name.clear();
		}

		let mut email = String::new();
		loop {
			print!("Email: ");
			io::stdout().flush().unwrap();
			io::stdin().read_line(&mut email)?;
			email = email.trim().to_string();
			if !email.is_empty() {
				break;
			}
			email.clear();
		}

		let mut date_str = String::new();

		let dob;
		loop {
			print!("Date of birth: ");
			io::stdout().flush().unwrap();
			date_str.clear();
			io::stdin().read_line(&mut date_str)?;
			if let Ok(d) = NaiveDate::parse_from_str(date_str.trim(), "%Y-%m-%d") {
				dob = d;
				break;
			}
			println!("Please enter a valid date in YYYY-MM-DD format");
		}

		let doj;
		loop {
			print!("Date of joining: ");
			io::stdout().flush().unwrap();
			date_str.clear();
			io::stdin().read_line(&mut date_str)?;
			if let Ok(d) = NaiveDate::parse_from_str(date_str.trim(), "%Y-%m-%d") {
				doj = d;
				break;
			}
			println!("Please enter a valid date in YYYY-MM-DD format");
		}

		let mut input_str = String::new();
		let gender;
		loop {
			input_str.clear();

			print!("Gender (m/f): ");
			io::stdout().flush().unwrap();
			io::stdin().read_line(&mut input_str)?;

			if let Ok(g) = Gender::try_from(input_str.trim()) {
				gender = g;
				break;
			}
			println!("Invalid input. Please enter 'm' or 'f' only");
		}

		let dep;
		loop {
			input_str.clear();

			print!("Department (leave it empty if none): ");
			io::stdout().flush().unwrap();
			io::stdin().read_line(&mut input_str)?;
			let input = input_str.trim();

			if input.is_empty() {
				dep = None;
				break;
			} else if let Ok(d) = input.parse::<u32>() {
				dep = Some(DepartmentId(d));
				break;
			}
			println!("Invalid input. Please enter depId or leave it empty");
		}

		let monthly_salary;
		loop {
			input_str.clear();

			print!("Monthly salary (leave it empty if not known): ");
			io::stdout().flush().unwrap();
			io::stdin().read_line(&mut input_str)?;
			let input = input_str.trim();

			if input.is_empty() {
				monthly_salary = None;
				break;
			} else if let Ok(s) = input.parse::<u32>() {
				monthly_salary = Some(s);
				break;
			}
			println!("Invalid input. Please enter an integer or leave it empty");
		}

		let builder =
			StaffBuilder { first_name, last_name, email, dob, doj, gender, department: dep, monthly_salary };

		self.execute(ctx, MenuItemInput::StaffBuilder(builder)).map(|_| ())
	}

	fn execute<'a>(
		&self,
		ctx: &'a mut Context,
		input: MenuItemInput,
	) -> Result<MenuItemOutput<'a>, Box<dyn error::Error>> {
		let MenuItemInput::StaffBuilder(builder) = input else {
			Err(ApplicationError("Unrecognized input".to_string()))?
		};

		Ok(ctx.insert_staff(builder).map(MenuItemOutput::Staff)?)
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
			Err(Box::new(ApplicationError("Unrecognized input".to_string())))?
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
			Err(Box::new(ApplicationError("Unrecognized input".to_string())))?
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
