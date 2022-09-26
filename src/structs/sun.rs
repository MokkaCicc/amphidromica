use crate::enums::Direction;
use crate::helpers::Period;

use super::Planet;

pub struct Sun<'a> {
	pub name: &'a str,
	pub radius: f64,
	pub mass: f64,
	pub revolution_period: Period,
	pub revolution_direction: Direction,
	pub planets: Vec<&'a Planet<'a>>,
}

impl<'a> Sun<'a> {
	pub fn new(
		name: &'a str,
		radius: f64,
		mass: f64,
		revolution_period: Period,
		revolution_direction: Direction,
	) -> Self {
		Self {
			name,
			radius,
			mass,
			revolution_period,
			revolution_direction,
			planets: Vec::new(),
		}
	}

	pub fn add_planet(&mut self, planet: &'a Planet<'a>) {
		self.planets.push(planet);
	}
}
