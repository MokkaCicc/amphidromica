use crate::helpers::Period;

pub trait Body<'a> {
	fn name(&self) -> &str;
	fn set_name(&mut self, name: &'a str);
	fn radius(&self) -> f64;
	fn set_radius(&mut self, radius: f64);
	fn mass(&self) -> f64;
	fn set_mass(&mut self, mass: f64);
	fn revolution_period(&self) -> Period;
	fn set_revolution_period(&mut self, period: Period);
}
