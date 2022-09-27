use crate::enums::Direction;

use super::Period;

pub struct Orbit {
	pub distance: f64,
	pub period: Period,
	pub direction: Direction,
}

impl Orbit {
	pub fn new(distance: f64, period: Period, direction: Direction) -> Self {
		Self {
			distance,
			period,
			direction,
		}
	}

	pub fn from_kepler_law(system_mass: f64, distance: f64, direction: Direction) -> Self {
		let period = Period::from_kepler_law(system_mass, distance);
		Self {
			distance,
			period,
			direction,
		}
	}
}
