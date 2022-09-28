mod enums;
mod helpers;
mod structs;

use enums::Direction;
use helpers::{Orbit, Period, Revolution};
use structs::Body;

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

	let orbits = vec![
		Orbit::from_kepler_law(&sun, &earth, 1.496e11, Direction::Prograde),
		Orbit::from_kepler_law(&earth, &moon, 3.844e8, Direction::Prograde),
	];

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
		println!();
	}

	// let hours = 69.0;
	// match earth.get_tidal_range() {
	// 	Some(range) => {
	// 		println!("{}", range);
	// 		println!("Tidal at {} hours : {:.3}m", hours, earth.tidal_at(hours));
	// 		println!();
	// 		for moon in &earth.moons {
	// 			println!("{} : ", moon.name);
	// 			println!("\tradius : {:10.3e}m", moon.radius);
	// 			println!("\tmass : {:10.3e}kg", moon.mass);
	// 			println!("\tdistance : {:10.3e}m", moon.orbit.distance);
	// 			println!("\tinfluence : {:.3}", moon.tidal_influence(&earth) / 2.0);
	// 			println!("\torbital period : {}", moon.orbit.period);
	// 			println!(
	// 				"\tsynodic period : {}",
	// 				earth.revolution.synodic_period(&moon.orbit)
	// 			);
	// 			println!("\tvisible diameter : {:.3}cm", moon.get_visible_diameter())
	// 		}
	// 	}
	// 	None => println!("{} does not have any moons", earth.name),
	// }
}
