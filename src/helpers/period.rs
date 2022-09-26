use std::f64::consts::PI;

use super::{physics::G, Time};
use crate::enums::Direction;

#[derive(Clone, Copy)]
pub struct Period {
	pub time: Time,
	pub direction: Direction,
}

impl Period {
	pub fn new(time: Time, direction: Direction) -> Self {
		Self { time, direction }
	}

	pub fn from_kepler_law(distance: f64, mass: f64, direction: Direction) -> Self {
		let period_seconds = 2.0 * PI * f64::sqrt(f64::powf(distance, 3.0) / (mass * G));
		let time = Time::from_seconds(period_seconds);
		Self { time, direction }
	}

	pub fn is_same_rotation_direction(&self, period: &Period) -> bool {
		self.direction == period.direction
	}
}
