pub struct DepartmentId(pub u32);

pub struct Department {
	pub id: DepartmentId,
	pub name: String,
	pub parent: Option<DepartmentId>,
}

impl Department {
	pub fn new(id: DepartmentId, name: &str, parent: Option<DepartmentId>) -> Self {
		Self { id, name: name.to_string(), parent }
	}
}
