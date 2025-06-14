use crate::traits::OneLiner;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepartmentId(pub u32);

impl From<u32> for DepartmentId {
	fn from(value: u32) -> Self {
		DepartmentId(value)
	}
}

impl From<DepartmentId> for u32 {
	fn from(did: DepartmentId) -> Self {
		did.0
	}
}

impl TryFrom<&str> for DepartmentId {
	type Error = &'static str;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		let value = value.parse::<u32>().map_err(|_| "Input unable to convert to DepartmentId")?;
		Ok(DepartmentId(value))
	}
}

/**
 * For Department type
 **/
#[derive(Clone, Debug, Getters, Setters, Serialize, Deserialize)]
pub struct Department {
	#[getset(get = "pub")]
	id: DepartmentId,

	#[getset(get = "pub")]
	name: String,

	#[getset(get = "pub")]
	parent: Option<DepartmentId>,
}

impl Department {
	pub fn new(id: DepartmentId, name: &str, parent: Option<DepartmentId>) -> Self {
		Self { id, name: name.to_string(), parent }
	}
}

impl OneLiner for Department {
	fn one_liner(&self) -> String {
		format!("{} department (id: {})", self.name, self.id.0)
	}
}

pub struct DepartmentInfo<'a> {
	pub department: &'a Department,
	pub headcount: u32,
	pub monthly_expense: u64,
}

impl fmt::Display for DepartmentInfo<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			r#"Department
Id: {}
Name: {}
Parent department: {}
Head count: {}
Monthly cost: {}"#,
			self.department.id().0,
			self.department.name(),
			match self.department.parent() {
				Some(parent_dep_id) => parent_dep_id.0.to_string(),
				None => "none".to_string(),
			},
			self.headcount,
			self.monthly_expense,
		)
	}
}
