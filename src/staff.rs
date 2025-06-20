use crate::department::DepartmentId;
use chrono::NaiveDate;
use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaffId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
	Male,
	Female,
}

#[derive(Clone, Debug, Getters, Setters, Serialize, Deserialize)]
#[getset(get = "pub")]
pub struct Staff {
	id: StaffId,
	first_name: String,
	last_name: String,
	email: String,
	// date of birth
	dob: NaiveDate,
	// date of joining the company
	doj: NaiveDate,
	// date of termination
	dot: Option<NaiveDate>,
	gender: Gender,
	department: Option<DepartmentId>,
	salary: Option<u32>,
	active: bool,
}

impl Staff {
	pub fn new(id: StaffId, builder: StaffBuilder) -> Staff {
		let StaffBuilder { first_name, last_name, email, dob, doj, dot, gender, department, salary, active } =
			builder;
		Staff {
			id,
			first_name,
			last_name,
			email,
			dob,
			doj,
			dot,
			gender,
			department,
			salary,
			active: active.unwrap_or(true),
		}
	}
}

pub struct StaffBuilder {
	pub first_name: String,
	pub last_name: String,
	pub email: String,
	pub dob: NaiveDate,
	pub doj: NaiveDate,
	pub dot: Option<NaiveDate>,
	pub gender: Gender,
	pub department: Option<DepartmentId>,
	pub salary: Option<u32>,
	pub active: Option<bool>,
}
