use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use crate::{
	department::{Department, DepartmentBuilder, DepartmentId, DepartmentInfo},
	errors::ApplicationError,
	staff::{Staff, StaffBuilder, StaffId},
};

#[derive(Clone, Debug, Getters, Setters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct Context {
	#[getset(set = "pub")]
	company_name: String,
	next_department_id: DepartmentId,
	next_staff_id: StaffId,
	departments: Vec<Department>,
	staff: Vec<Staff>,
}

impl Context {
	pub fn new() -> Self {
		Self {
			company_name: "".to_string(),
			next_department_id: DepartmentId(0),
			next_staff_id: StaffId(0),
			departments: vec![],
			staff: vec![],
		}
	}

	fn get_next_department_id(&mut self) -> DepartmentId {
		let ret = self.next_department_id;
		self.next_department_id = DepartmentId(self.next_department_id.0 + 1);

		ret
	}

	fn validate_department_id(&self, dep_id: &DepartmentId) -> bool {
		self.departments.iter().any(|dep| dep.id() == dep_id)
	}

	pub fn insert_department(&mut self, builder: DepartmentBuilder) -> Result<&Department, ApplicationError> {
		// builder parameter validation
		if let Some(dep_id) = builder.parent {
			if !self.validate_department_id(&dep_id) {
				Err(ApplicationError("Unknown department".to_string()))?
			}
		};

		let new_department = Department::new(self.get_next_department_id(), builder);
		self.departments.push(new_department);

		Ok(self.departments.last().unwrap())
	}

	pub fn department(&self, dep_id: &DepartmentId) -> Option<&Department> {
		self.departments().iter().filter(|dep| *dep.id() == *dep_id).collect::<Vec<_>>().first().copied()
	}

	fn get_next_staff_id(&mut self) -> StaffId {
		let ret = self.next_staff_id;
		self.next_staff_id = StaffId(self.next_staff_id.0 + 1);

		ret
	}

	pub fn department_info(&self, dep_id: &DepartmentId) -> Option<DepartmentInfo> {
		let dep = self.department(dep_id)?;
		let headcount = 0;
		let monthly_expense = 0;

		Some(DepartmentInfo { department: dep, headcount, monthly_expense })
	}

	pub fn insert_staff(&mut self, builder: StaffBuilder) -> Result<&Staff, ApplicationError> {
		// TODO: validate the staff builder info in the context

		let new_staff = Staff::new(self.get_next_staff_id(), builder);
		self.staff.push(new_staff);

		Ok(self.staff.last().unwrap())
	}
}
