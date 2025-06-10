use crate::department::{Department, DepartmentId};
use crate::staff::Staff;

pub struct Context {
	company_name: String,
	next_department_id: DepartmentId,
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
}
