use crate::helpers::Revolution;

use super::Planet;

pub struct Sun<'a> {
	pub name: &'a str,
	pub radius: f64,
	pub mass: f64,
	pub revolution: Revolution,
	pub planets: Vec<&'a Planet<'a>>,
}

impl<'a> Sun<'a> {
	pub fn new(name: &'a str, radius: f64, mass: f64, revolution: Revolution) -> Self {
		Self {
			name,
			radius,
			mass,
			revolution,
			planets: Vec::new(),
		}
	}

	pub fn add_planet(&mut self, planet: &'a Planet<'a>) {
		self.planets.push(planet);
	}
}
