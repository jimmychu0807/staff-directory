use crate::traits::OneLiner;
use getset::{Getters, Setters};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DepartmentId(pub u32);

impl From<u32> for DepartmentId {
	fn from(value: u32) -> Self {
		DepartmentId(value)
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
#[derive(Clone, Debug, Getters, Setters)]
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
