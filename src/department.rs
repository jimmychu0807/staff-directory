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
#[getset(get = "pub")]
pub struct Department {
	id: DepartmentId,
	name: String,
	parent: Option<DepartmentId>,
	active: bool,
}

impl Department {
	pub fn new(id: DepartmentId, builder: DepartmentBuilder) -> Self {
		let DepartmentBuilder { name, parent, active } = builder;
		Department { id, name, parent, active: active.unwrap_or(true) }
	}
}

impl OneLiner for Department {
	fn one_liner(&self) -> String {
		format!("{} department (id: {})", self.name, self.id.0)
	}
}

#[derive(Debug)]
pub struct DepartmentBuilder {
	pub name: String,
	pub parent: Option<DepartmentId>,
	pub active: Option<bool>,
}

impl DepartmentBuilder {
	pub fn new(name: String, parent: Option<DepartmentId>) -> Self {
		DepartmentBuilder { name, parent, active: Some(true) }
	}
}

#[derive(Debug)]
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
