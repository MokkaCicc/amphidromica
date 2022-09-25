use super::Range;
use std::fmt;

pub const TIDAL_MULTS: (f64, f64, f64) = (
	1.921, // from 0 to 2m on earth
	3.842, // from 2m to 4m on earth
	7.683, // from 4m to 8m on earth
);

#[derive(Debug)]
pub struct TidalRange {
	// base tidal range, most common in the open sea
	pub tidal_range: f64,

	// very low tidal range (e.g. the mediterranean sea)
	pub micro_tidal: Range,

	// normal tidal range, most common
	pub meso_tidal: Range,

	// high tidal range, (e.g. Brittany)
	pub macro_tidal: Range,

	// very high tidal range (e.g. the bay of Fundy)
	pub mega_tidal: Range,
}

impl TidalRange {
	pub fn new(tidal_range: f64) -> Self {
		let tidal_range = tidal_range;
		let micro_tidal = Range::new(None, Some(tidal_range * TIDAL_MULTS.0));
		let meso_tidal = Range::new(
			Some(tidal_range * TIDAL_MULTS.0),
			Some(tidal_range * TIDAL_MULTS.1),
		);
		let macro_tidal = Range::new(
			Some(tidal_range * TIDAL_MULTS.1),
			Some(tidal_range * TIDAL_MULTS.2),
		);
		let mega_tidal = Range::new(Some(tidal_range * TIDAL_MULTS.2), None);

		Self {
			tidal_range,
			micro_tidal,
			meso_tidal,
			macro_tidal,
			mega_tidal,
		}
	}
}

impl fmt::Display for TidalRange {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}\n{}\n{}\n{}\n{}\n{}",
			format!("Tidal range : {:.3}m", self.tidal_range),
			format!("Tidal amplitude : {:.3}m", self.tidal_range / 2.0),
			format!("Micro tidal : {}", self.micro_tidal),
			format!("Meso tidal : {}", self.meso_tidal),
			format!("Macro tidal : {}", self.macro_tidal),
			format!("Mega tidal : {}", self.mega_tidal),
		)
	}
}
