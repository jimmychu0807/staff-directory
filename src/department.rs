use crate::traits::MenuItem;

pub struct ListDepartments();

impl ListDepartments {
	pub fn new() -> Self {
		Self()
	}
}

impl MenuItem for ListDepartments {
	fn menuitem_txt(&self) -> &str {
		"List department hierarchy"
	}

	fn execute(&self) {
		println!("executing ListDepartments");
	}
}
