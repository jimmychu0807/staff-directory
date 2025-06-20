use super::*;
use crate::{
	context::Context,
	department::{DepartmentBuilder, DepartmentId},
	menu_items::MenuItemInput,
};

#[test]
fn name_a_company() {
	let mut ctx = Context::new();
	let name_company: Box<dyn MenuItem> = Box::new(NameCompany::new());

	let compay_name = String::from("MyCompany");
	let _ = name_company.execute(&mut ctx, MenuItemInput::String(compay_name.clone()));

	assert_eq!(*ctx.company_name(), compay_name);
}

#[test]
fn create_a_department() {
	let mut ctx = Context::new();
	let create_department: Box<dyn MenuItem> = Box::new(CreateDepartment::new());

	let dept1_params = MenuItemInput::DepartmentBuilder(DepartmentBuilder::new("Dept1".to_string(), None));
	let _ = create_department.execute(&mut ctx, dept1_params);

	let next_dep_id = ctx.next_department_id();
	let departments = ctx.departments();

	assert_eq!(*next_dep_id, DepartmentId(1));
	assert_eq!(departments.len(), 1);

	let dept1 = &departments[0];
	assert_eq!(*dept1.id(), DepartmentId(0));
	assert_eq!(*dept1.name(), "Dept1".to_string());
	assert_eq!(*dept1.parent(), None);
}
