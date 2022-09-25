use crate::helpers::{Period, TidalRange};

use super::Moon;

pub struct Planet<'a> {
	// name of the planet
	pub name: &'a str,

	// radius of the planet, in meters
	pub radius: f64,

	// mass of the planet, in kilogramms
	pub mass: f64,

	// period of one full revolution of the planet in days, hours and minutes
	pub rotation_period: Period,

	// the moons of the planet
	pub moons: Vec<&'a Moon<'a>>,

	// the tidal ranges of this planet
	tidal_range: Option<TidalRange>,

	// if the tidal ranges are up to date.
	is_tidal_range_updated: bool,
}

impl<'a> Planet<'a> {
	pub fn new(name: &'a str, radius: f64, mass: f64, rotation_period: Period) -> Self {
		Self {
			name,
			radius,
			mass,
			rotation_period,
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
