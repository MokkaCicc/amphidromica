use std::fmt;

#[derive(Debug)]
pub struct Range {
	pub min: Option<f64>,
	pub max: Option<f64>,
}

impl Range {
	pub fn new(min: Option<f64>, max: Option<f64>) -> Self {
		if min.is_none() && max.is_none() {
			panic!("min and max cannot be None at the same time");
		}
		if min > max {
			Self { min, max }
		} else {
			Self { max, min }
		}
	}
}

impl fmt::Display for Range {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.min {
			Some(min) => match self.max {
				Some(max) => write!(f, "{:.3}m < range < {:.3}m", min, max),
				None => write!(f, "range < {:.3}m", min),
			},
			None => match self.max {
				Some(max) => write!(f, "range > {:.3}m", max),
				None => write!(f, "impossible..."),
			},
		}
	}
}
