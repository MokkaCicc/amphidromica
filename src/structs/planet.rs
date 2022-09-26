use crate::helpers::{Period, TidalRange};
use crate::traits::Body;

use super::Moon;

pub struct Planet<'a> {
	name: &'a str,
	radius: f64,
	mass: f64,
	revolution_period: Period,

	// the moons of the planet
	pub moons: Vec<&'a Moon<'a>>,

	// the tidal ranges of this planet
	tidal_range: Option<TidalRange>,

	// if the tidal ranges are up to date.
	is_tidal_range_updated: bool,
}

impl<'a> Planet<'a> {
	pub fn new(name: &'a str, radius: f64, mass: f64, revolution_period: Period) -> Self {
		Self {
			name,
			radius,
			mass,
			revolution_period,
			moons: Vec::new(),
			tidal_range: None,
			is_tidal_range_updated: true,
		}
	}

	pub fn add_moon(&mut self, moon: &'a Moon<'a>) {
		self.moons.push(moon);
		self.is_tidal_range_updated = false;
	}

	pub fn get_tidal_range(&mut self) -> Option<&TidalRange> {
		if !self.is_tidal_range_updated {
			self.update_tidal_range();
		}

		self.tidal_range.as_ref()
	}

	pub fn get_tidal_at(&self, hours: f64) -> f64 {
		let mut tidals = Vec::new();
		for moon in self.moons.to_owned() {
			tidals.push(moon.get_tidal_at(hours));
		}
		tidals.into_iter().sum()
	}

	fn update_tidal_range(&mut self) {
		if self.moons.is_empty() {
			self.tidal_range = None;
			self.is_tidal_range_updated = true;
			return;
		}

		let mut tidal_amplitudes = Vec::new();
		for moon in self.moons.to_owned() {
			tidal_amplitudes.push(moon.tidal_influence)
		}
		let base_tidal = tidal_amplitudes.into_iter().sum();
		self.tidal_range = Some(TidalRange::new(base_tidal));
		self.is_tidal_range_updated = true;
	}
}

impl<'a> Body<'a> for Planet<'a> {
	fn name(&self) -> &str {
		self.name
	}

	fn set_name(&mut self, name: &'a str) {
		self.name = name;
	}

	fn radius(&self) -> f64 {
		self.radius
	}

	fn set_radius(&mut self, radius: f64) {
		self.radius = radius;
	}

	fn mass(&self) -> f64 {
		self.mass
	}

	fn set_mass(&mut self, mass: f64) {
		self.mass = mass;
	}

	fn revolution_period(&self) -> Period {
		self.revolution_period
	}

	fn set_revolution_period(&mut self, period: Period) {
		self.revolution_period = period;
	}
}
