#![allow(dead_code, unused_variables)]
use std::f64::consts::PI;
use std::fmt;

use super::physics::G;

const DAY_TO_HOURS: f64 = 24.0;
const DAY_TO_MINUTES: f64 = DAY_TO_HOURS * HOUR_TO_MINUTES;
const DAY_TO_SECONDES: f64 = DAY_TO_HOURS * HOUR_TO_MINUTES * MINUTE_TO_SECONDES;
const HOUR_TO_DAYS: f64 = 1.0 / DAY_TO_HOURS;
const HOUR_TO_MINUTES: f64 = 60.0;
const HOUR_TO_SECONDES: f64 = HOUR_TO_MINUTES * MINUTE_TO_SECONDES;
const MINUTE_TO_DAYS: f64 = 1.0 / DAY_TO_MINUTES;
const MINUTE_TO_HOURS: f64 = 1.0 / HOUR_TO_MINUTES;
const MINUTE_TO_SECONDES: f64 = 60.0;
const SECONDE_TO_DAYS: f64 = 1.0 / DAY_TO_SECONDES;
const SECONDE_TO_HOURS: f64 = 1.0 / HOUR_TO_SECONDES;
const SECONDE_TO_MINUTES: f64 = 1.0 / MINUTE_TO_SECONDES;

#[derive(Debug)]
pub struct Period {
	pub d: f64, // days
	pub h: f64, // hours
	pub m: f64, // minutes
	pub s: f64, // seconds
}

impl Period {
	pub fn new(d: i32, h: i32, m: i32, s: i32) -> Self {
		Self {
			d: d as f64,
			h: h as f64,
			m: m as f64,
			s: s as f64,
		}
	}

	pub fn from_kepler_law(system_mass: f64, distance: f64) -> Self {
		let period_seconds = 2.0 * PI * f64::sqrt(f64::powf(distance, 3.0) / (system_mass * G));
		Period::from_seconds(period_seconds)
	}

	pub fn from_days(days: f64) -> Self {
		unimplemented!()
	}

	pub fn from_hours(hours: f64) -> Self {
		unimplemented!()
	}

	pub fn from_minutes(minutes: f64) -> Self {
		unimplemented!()
	}

	pub fn from_seconds(seconds: f64) -> Self {
		let d = (seconds / DAY_TO_SECONDES).floor();
		let mut r = seconds % DAY_TO_SECONDES;
		let h = (r / HOUR_TO_SECONDES).floor();
		r = r % HOUR_TO_SECONDES;
		let m = (r / MINUTE_TO_SECONDES).floor();
		let s = r % MINUTE_TO_SECONDES;

		Period::new(d as i32, h as i32, m as i32, s as i32)
	}

	pub fn into_days(&self) -> f64 {
		self.d * DAY_TO_HOURS
			+ self.h * HOUR_TO_DAYS
			+ self.m * MINUTE_TO_DAYS
			+ self.s * SECONDE_TO_DAYS
	}

	pub fn into_hours(&self) -> f64 {
		self.d * DAY_TO_HOURS + self.h + self.m * MINUTE_TO_HOURS + self.s * SECONDE_TO_HOURS
	}

	pub fn into_minutes(&self) -> f64 {
		self.s * DAY_TO_MINUTES + self.h * HOUR_TO_MINUTES + self.m + self.s * SECONDE_TO_MINUTES
	}

	pub fn into_seconds(&self) -> f64 {
		self.d * DAY_TO_SECONDES + self.h * HOUR_TO_SECONDES + self.m * MINUTE_TO_SECONDES + self.s
	}
}

impl fmt::Display for Period {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}d {}h {}m {}s", self.d, self.h, self.m, self.s)
	}
}
