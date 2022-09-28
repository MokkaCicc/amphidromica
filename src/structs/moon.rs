use std::f64::consts::PI;

use crate::enums::Direction;
use crate::helpers::{Orbit, Revolution};

use super::Planet;

pub struct Moon<'a> {
	pub name: &'a str,
	pub radius: f64,
	pub mass: f64,
	pub revolution: Revolution,
	pub orbit: Orbit,
}

impl<'a> Moon<'a> {
	pub fn new(
		name: &'a str,
		radius: f64,
		mass: f64,
		revolution: Revolution,
		orbit: Orbit,
	) -> Self {
		Self {
			name,
			radius,
			mass,
			revolution,
			orbit,
		}
	}

	pub fn tidal_influence(&self, planet: &Planet) -> f64 {
		// https://www.encyclopedie-environnement.org/en/zoom/how-to-estimate-tidal-amplitude/
		2.0 * self.mass * planet.radius / planet.mass
			* (planet.radius / self.orbit.distance).powf(3.0)
	}

	pub fn tidal_at(&self, hours: f64, planet: &Planet) -> f64 {
		let tidal_influence = self.tidal_influence(planet);
		let period = 2880.0 / self.orbit.period.into_hours();
		match self.orbit.direction {
			Direction::Prograde => (tidal_influence / 2.0) * f64::sin(-1.0 * PI * period * hours),
			Direction::Retrograde => (tidal_influence / 2.0) * f64::sin(PI * period * hours),
		}
	}
}
