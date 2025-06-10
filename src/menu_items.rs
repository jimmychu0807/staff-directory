use crate::context::Context;

pub trait MenuItem {
	fn menuitem_txt(&self) -> &str;
	fn hotkey(&self) -> &str;
	fn execute(&self, ctx: &mut Context);
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

	fn hotkey(&self) -> &str {
		&self.hotkey
	}

	fn execute(&self, ctx: &mut Context) {
		println!("Naming the company");
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

	fn hotkey(&self) -> &str {
		&self.hotkey
	}

	fn execute(&self, ctx: &mut Context) {
		println!("listing departments");
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

	fn hotkey(&self) -> &str {
		&self.hotkey
	}

	fn execute(&self, ctx: &mut Context) {
		println!("create a new department");
	}
}
