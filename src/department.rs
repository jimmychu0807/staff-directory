use crate::traits::MenuItem;

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

	fn execute(&self) {
		println!("listing departments");
	}
}

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

	fn execute(&self) {
		println!("create a new department");
	}
}
