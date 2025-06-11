use crate::traits::OneLiner;
use getset::{Getters, Setters};

#[derive(Clone, Debug)]
pub struct DepartmentId(pub u32);

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
