use getset::{Getters, Setters};

use crate::department::{Department, DepartmentId};
use crate::staff::Staff;

#[derive(Clone, Debug, Getters, Setters)]
pub struct Context {
	#[getset(get = "pub", set = "pub")]
	company_name: String,

	next_department_id: DepartmentId,

	#[getset(get = "pub")]
	departments: Vec<Department>,
	staff: Vec<Staff>,
}

impl Context {
	pub fn new() -> Self {
		Self {
			company_name: "".to_string(),
			next_department_id: DepartmentId(0),
			departments: vec![],
			staff: vec![],
		}
	}

	pub fn get_next_department_id(&mut self) -> DepartmentId {
		let ret = self.next_department_id;
		self.next_department_id = DepartmentId(self.next_department_id.0 + 1);

		ret
	}

	pub fn insert_department(&mut self, new_department: Department) {
		self.departments.push(new_department);
	}

	pub fn validate_department_id(&self, dep_id: &DepartmentId) -> bool {
		self.departments.iter().any(|dep| dep.id() == dep_id)
	}
}
