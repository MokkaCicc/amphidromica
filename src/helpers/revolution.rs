use crate::enums::Direction;

use super::{Orbit, Period};

pub struct Revolution {
	pub period: Period,
	pub direction: Direction,
}

impl Revolution {
	pub fn new(period: Period, direction: Direction) -> Self {
		Self { period, direction }
	}

	pub fn synodic_period(&self, orbit: &Orbit) -> Period {
		let revolution_period_seconds = self.period.into_seconds();
		let orbit_period_seconds = orbit.period.into_seconds();

		// moon day is calculated with this formula :
		// abs(moon_period ± planet_period) / moon_period
		// where - is used when the planet and the moon have the same rotation direction
		let synodic_period_seconds = if orbit.direction == Direction::Prograde {
			orbit_period_seconds * revolution_period_seconds
				/ (orbit_period_seconds - revolution_period_seconds).abs()
		} else {
			// (moon_period + planet_period).abs() / moon_period
			orbit_period_seconds * revolution_period_seconds
				/ (orbit_period_seconds + revolution_period_seconds)
		};

		Period::from_seconds(synodic_period_seconds)
	}
}
