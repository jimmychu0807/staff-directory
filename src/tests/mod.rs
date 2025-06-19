use super::*;
use crate::context::Context;

#[test]
fn name_a_company() {
	let mut ctx = Context::new();
	let name_company: Box<dyn MenuItem> = Box::new(NameCompany::new());
}
