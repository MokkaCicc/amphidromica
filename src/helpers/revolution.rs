use crate::enums::Direction;

use super::Period;

pub struct Revolution {
	pub period: Period,
	pub direction: Direction,
}

impl Revolution {
	pub fn new(period: Period, direction: Direction) -> Self {
		Self { period, direction }
	}
}
