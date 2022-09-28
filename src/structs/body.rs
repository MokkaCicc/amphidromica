use crate::helpers::Revolution;

pub struct Body<'a> {
	pub name: &'a str,
	pub radius: f64,
	pub mass: f64,
	pub revolution: Revolution,
}

impl<'a> Body<'a> {
	pub fn new(name: &'a str, radius: f64, mass: f64, revolution: Revolution) -> Self {
		Self {
			name,
			radius,
			mass,
			revolution,
		}
	}
}
