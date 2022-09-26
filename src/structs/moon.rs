use std::f64::consts::PI;

use super::Planet;
use crate::enums::Direction;
use crate::helpers::Period;

pub struct Moon<'a> {
	pub name: &'a str,
	pub radius: f64,
	pub mass: f64,
	pub revolution_period: Period,
	pub revolution_direction: Direction,
	pub orbit_period: Period,
	pub orbit_direction: Direction,
	pub orbit_distance: f64,

	pub tidal_influence: f64,
}

impl<'a> Moon<'a> {
	pub fn new(
		name: &'a str,
		radius: f64,
		mass: f64,
		revolution_period: Period,
		revolution_direction: Direction,
		orbit_direction: Direction,
		orbit_distance: f64,
		planet_radius: f64,
		planet_mass: f64,
	) -> Self {
		let orbit_period = Period::from_kepler_law(planet_mass + mass, orbit_distance);
		// https://www.encyclopedie-environnement.org/en/zoom/how-to-estimate-tidal-amplitude/
		let tidal_influence =
			2.0 * mass * planet_radius / planet_mass * (planet_radius / orbit_distance).powf(3.0);
		Self {
			name,
			radius,
			mass,
			revolution_period,
			revolution_direction,
			orbit_period,
			orbit_direction,
			orbit_distance,
			tidal_influence,
		}
	}

	pub fn get_synodic_period_seconds(&self, planet: &Planet) -> f64 {
		let moon_period = self.orbit_period.into_seconds();
		let planet_period = planet.revolution_period.into_seconds();

		// moon day is calculated with this formula :
		// abs(moon_period ± planet_period) / moon_period
		// where - is used when the planet and the moon have the same rotation direction
		if self.orbit_direction == planet.revolution_direction {
			moon_period * planet_period / (moon_period - planet_period).abs()
		} else {
			// (moon_period + planet_period).abs() / moon_period
			moon_period * planet_period / (moon_period + planet_period).abs()
		}
	}

	// the time separating one lunar zenith from the next
	pub fn get_synodic_period(&self, planet: &Planet) -> Period {
		let synodic_period_seconds = self.get_synodic_period_seconds(planet);
		Period::from_seconds(synodic_period_seconds)
	}

	pub fn get_tidal_at(&self, hours: f64) -> f64 {
		let period = 2880.0 / self.orbit_period.into_hours();
		match self.orbit_direction {
			Direction::Prograde => {
				(self.tidal_influence / 2.0) * f64::sin(-1.0 * PI * period * hours)
			}
			Direction::Retrograde => (self.tidal_influence / 2.0) * f64::sin(PI * period * hours),
		}
	}

	// get the diameter from this moon as seen from the planet
	// the size in mesured is in centimeters measured at arm's length (0.75m)
	pub fn get_visible_diameter(&self) -> f64 {
		let size = 0.75 * 2.0 * self.radius / self.orbit_distance;
		size * 100.0
	}
}
