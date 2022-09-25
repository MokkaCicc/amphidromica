use std::f64::consts::PI;

use super::{physics::G, Time};
use crate::enums::Rotation;

pub struct Period {
	pub time: Time,
	pub rotation: Rotation,
}

impl Period {
	pub fn new(time: Time, rotation: Rotation) -> Self {
		Self { time, rotation }
	}

	pub fn from_kepler_law(distance: f64, mass: f64, rotation: Rotation) -> Self {
		let period_seconds = 2.0 * PI * f64::sqrt(f64::powf(distance, 3.0) / (mass * G));
		let time = Time::from_seconds(period_seconds);
		Self { time, rotation }
	}

	pub fn is_same_rotation_direction(&self, period: &Period) -> bool {
		self.rotation == period.rotation
	}
}
