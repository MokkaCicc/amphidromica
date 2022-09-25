use std::f64::consts::PI;

use super::Planet;
use crate::{
	enums::Rotation,
	helpers::{Period, Time},
};

pub struct Moon<'a> {
	// name of the moon
	pub name: &'a str,

	// radius of the moon, in meters
	pub radius: f64,

	// mass of the moon, in kilogramms
	pub mass: f64,

	// distance from its planet, in meters
	pub distance: f64,

	pub orbital_period: Period,

	// the tidal influence to the planet, in meters
	pub tidal_influence: f64,
}

impl<'a> Moon<'a> {
	pub fn new(
		name: &'a str,
		radius: f64,
		mass: f64,
		distance: f64,
		rotation: Rotation,
		planet: (f64, f64),
	) -> Self {
		Self {
			name,
			radius,
			mass,
			distance,
			orbital_period: Period::from_kepler_law(distance, planet.1 + mass, rotation),
			// https://www.encyclopedie-environnement.org/en/zoom/how-to-estimate-tidal-amplitude/
			tidal_influence: 2.0 * mass * planet.0 / planet.1 * (planet.0 / distance).powf(3.0),
		}
	}

	pub fn get_synodic_period_seconds(&self, planet: &Planet) -> f64 {
		let moon_period = self.orbital_period.time.into_seconds();
		let planet_period = planet.rotation_period.time.into_seconds();

		// moon day is calculated with this formula :
		// abs(moon_period ± planet_period) / moon_period
		// where - is used when the planet and the moon have the same rotation direction
		if self
			.orbital_period
			.is_same_rotation_direction(&planet.rotation_period)
		{
			moon_period * planet_period / (moon_period - planet_period).abs()
		} else {
			// (moon_period + planet_period).abs() / moon_period
			moon_period * planet_period / (moon_period + planet_period).abs()
		}
	}

	// the time separating one lunar zenith from the next
	pub fn get_synodic_period(&self, planet: &Planet) -> Time {
		let synodic_period_seconds = self.get_synodic_period_seconds(planet);
		Time::from_seconds(synodic_period_seconds)
	}

	pub fn get_tidal_at(&self, hours: f64) -> f64 {
		let period = 2880.0 / self.orbital_period.time.into_hours();
		match self.orbital_period.rotation {
			Rotation::Prograde => {
				(self.tidal_influence / 2.0) * f64::sin(-1.0 * PI * period * hours)
			}
			Rotation::Retrograde => (self.tidal_influence / 2.0) * f64::sin(PI * period * hours),
		}
	}

	// get the diameter from this moon as seen from the planet
	// the size in mesured is in centimeters measured at arm's length (0.75m)
	pub fn get_visible_diameter(&self) -> f64 {
		let size = 0.75 * 2.0 * self.radius / self.distance;
		size * 100.0
	}
}
