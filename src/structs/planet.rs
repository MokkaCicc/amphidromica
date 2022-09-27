use crate::helpers::{Orbit, Revolution, TidalRange};

use super::Moon;

pub struct Planet<'a> {
	pub name: &'a str,
	pub radius: f64,
	pub mass: f64,
	pub revolution: Revolution,
	pub orbit: Orbit,
	pub moons: Vec<&'a Moon<'a>>,

	tidal_range: Option<TidalRange>,
	is_tidal_range_updated: bool,
}

impl<'a> Planet<'a> {
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

	pub fn tidal_at(&self, hours: f64) -> f64 {
		let mut tidals = Vec::new();
		for moon in self.moons.to_owned() {
			tidals.push(moon.tidal_at(hours, &self));
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
			tidal_amplitudes.push(moon.tidal_influence(&self))
		}
		let base_tidal = tidal_amplitudes.into_iter().sum();
		self.tidal_range = Some(TidalRange::new(base_tidal));
		self.is_tidal_range_updated = true;
	}
}
