use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Direction {
	// for an obital rotation, the motion of a body is prograde when it orbit in the same direction of the spin of the body it orbits
	// for a body spin, the motion is retrograde when the body is spining in the same direction of it orbit
	Prograde,

	// for an obital rotation, the motion of a body is retrograde when it orbit in the opposite direction of the spin of the body it orbits
	// for a body spin, the motion is retrograde when the body is spining in the opposite direction of it orbit
	Retrograde,
}

impl fmt::Display for Direction {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let direction = match self {
			Direction::Prograde => "Prograde",
			Direction::Retrograde => "Retrograde",
		};
		write!(f, "{}", direction)
	}
}
