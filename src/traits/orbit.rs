use crate::helpers::Period;

use super::Body;

pub trait Orbit {
	fn orbit_period(&self) -> Period;
	fn set_orbit_period(&mut self, period: Period);
	fn is_prograde<'a>(&self, revolver: impl Body<'a>) -> bool {
		self.orbit_period().direction == revolver.revolution_period().direction
	}
}
