use crate::enums::Direction;
use crate::structs::Body;

use super::Period;

const ARM_LENGTH: f64 = 0.75;

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

	fn synodic_period(
		revolution_period: &Period,
		orbit_period: &Period,
		orbit_direction: &Direction,
	) -> Period {
		let revolution_seconds = revolution_period.into_seconds();
		let orbit_seconds = orbit_period.into_seconds();

		// moon day is calculated with this formula :
		// abs(orbit_period ± primary_period) / orbit_period
		// where - is used when the primary and the satellite have the same rotation direction
		let synodic_period_seconds = if orbit_direction == &Direction::Prograde {
			orbit_seconds * revolution_seconds / (orbit_seconds - revolution_seconds).abs()
		} else {
			orbit_seconds * revolution_seconds / (orbit_seconds + revolution_seconds)
		};

		Period::from_seconds(synodic_period_seconds)
	}

	pub fn synodic_period_primary(&self) -> Period {
		Orbit::synodic_period(
			&self.satellite.revolution.period,
			&self.period,
			&self.direction,
		)
	}

	pub fn synodic_period_satellite(&self) -> Period {
		Orbit::synodic_period(
			&self.primary.revolution.period,
			&self.period,
			&self.direction,
		)
	}

	// get the diameter from this moon as seen from the planet
	// the size in mesured is in centimeters measured at arm's length (0.75m)
	pub fn visible_diameter_primary(&self) -> f64 {
		let size = ARM_LENGTH * 2.0 * self.primary.radius / self.radius;
		size * 100.0
	}

	pub fn visible_diameter_satellite(&self) -> f64 {
		let size = ARM_LENGTH * 2.0 * self.satellite.radius / self.radius;
		size * 100.0
	}
}
