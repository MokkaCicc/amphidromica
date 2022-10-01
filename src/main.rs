mod enums;
mod helpers;
mod structs;

use enums::Direction;
use helpers::{Orbit, Period, Revolution};
use structs::Body;

use crate::helpers::TidalRange;

fn main() {
	let sun = Body::new(
		"Sun",
		6.963e8,
		1.989e30,
		Revolution::new(Period::new(27, 6, 43, 12), Direction::Prograde),
	);
	let earth = Body::new(
		"Earth",
		6.371e6,
		5.973e24,
		Revolution::new(Period::new(1, 0, 0, 0), Direction::Prograde),
	);
	let moon = Body::new(
		"Moon",
		1.737e6,
		7.348e22,
		Revolution::new(Period::new(27, 7, 42, 14), Direction::Prograde),
	);
	let bodies = vec![&sun, &earth, &moon];

	let sun_earth = Orbit::from_kepler_law(&sun, &earth, 1.496e11, Direction::Prograde);
	let earth_moon = Orbit::from_kepler_law(&earth, &moon, 3.844e8, Direction::Prograde);

	let orbits = vec![&sun_earth, &earth_moon];

	for body in bodies {
		println!("{} :", body.name);
		println!("\tradius : {:10.3e} m", body.radius);
		println!("\tmass : {:10.3e} kg", body.mass);
		println!("\trevolution : {}", body.revolution.period);
		println!("\tdirection : {}", body.revolution.direction);
		println!();
	}

	for orbit in orbits {
		println!(
			"{} is orbiting {} :",
			orbit.satellite.name, orbit.primary.name
		);
		println!("\tradius : {:10.3e} m", orbit.radius);
		println!("\tperiod : {}", orbit.period);
		println!("\tdirection : {}", orbit.direction);
		println!(
			"\tsynodic period (primary) : {}",
			orbit.synodic_period_primary()
		);
		println!(
			"\tsynodic period (satellite) : {}",
			orbit.synodic_period_satellite()
		);
		println!(
			"\tvisible diameter (primary) : {:.3}cm",
			orbit.visible_diameter_primary()
		);
		println!(
			"\tvisible diameter (satellite) : {:.3}cm",
			orbit.visible_diameter_satellite()
		);
		println!(
			"\ttidal influance (primary) : {:.3}m",
			orbit.tidal_influance_primary()
		);
		println!(
			"\ttidal influance (satellite) : {:.3}m",
			orbit.tidal_influance_satellite()
		);
		println!();
	}

	let tidal_ranges = vec![
		sun_earth.tidal_influance_primary(),
		earth_moon.tidal_influance_satellite(),
	];
	println!("Tidal on earth : ");
	println!("{}", TidalRange::from_amplitudes(tidal_ranges));
}
