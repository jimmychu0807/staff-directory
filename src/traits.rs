pub trait MenuItem {
	fn menuitem_txt(&self) -> &str;
	fn hotkey(&self) -> &str;
	fn execute(&self);
}
