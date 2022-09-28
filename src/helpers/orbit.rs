use crate::enums::Direction;
use crate::structs::Body;

use super::Period;

pub struct Orbit<'a> {
	pub primary: &'a Body<'a>,
	pub satellite: &'a Body<'a>,
	pub radius: f64,
	pub period: Period,
	pub direction: Direction,
}

impl<'a> Orbit<'a> {
	pub fn new(
		primary: &'a Body<'a>,
		satellite: &'a Body<'a>,
		radius: f64,
		period: Period,
		direction: Direction,
	) -> Self {
		Self {
			primary,
			satellite,
			radius,
			period,
			direction,
		}
	}

	pub fn from_kepler_law(
		primary: &'a Body<'a>,
		satellite: &'a Body<'a>,
		radius: f64,
		direction: Direction,
	) -> Self {
		let period = Period::from_kepler_law(primary.mass + satellite.mass, radius);
		Self {
			primary,
			satellite,
			radius,
			period,
			direction,
		}
	}
}
