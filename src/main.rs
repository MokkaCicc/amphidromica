mod enums;
mod helpers;
mod structs;
mod traits;

use enums::Direction;
use helpers::Period;
use structs::{Moon, Planet, Sun};

fn main() {
	let mut sun = Sun::new(
		"Sun",
		6.963e8,
		1.989e30,
		Period::new(27, 6, 43, 12),
		Direction::Prograde,
	);
	let mut earth = Planet::new(
		"Earth",
		6.371e6,
		5.973e24,
		Period::new(1, 0, 0, 0),
		Direction::Prograde,
		Direction::Prograde,
		1.496e11,
		sun.mass,
	);
	let moon = Moon::new(
		"Moon",
		1.737e6,
		7.348e22,
		Period::new(27, 7, 42, 14),
		Direction::Prograde,
		Direction::Prograde,
		3.844e8,
		earth.radius,
		earth.mass,
	);

	sun.add_planet(&earth);
	earth.add_moon(&moon);

	let hours = 69.0;
	match earth.get_tidal_range() {
		Some(range) => {
			println!("{}", range);
			println!(
				"Tidal at {} hours : {:.3}m",
				hours,
				earth.get_tidal_at(hours)
			);
			println!();
			for moon in &earth.moons {
				println!("{} : ", moon.name);
				println!("\tradius : {:10.3e}m", moon.radius);
				println!("\tmass : {:10.3e}kg", moon.mass);
				println!("\tdistance : {:10.3e}m", moon.orbit_distance);
				println!("\tinfluence : {:.3}", moon.tidal_influence / 2.0);
				println!("\torbital period : {}", moon.orbit_period);
				println!("\tsynodic period : {}", moon.get_synodic_period(&earth));
				println!("\tvisible diameter : {:.3}cm", moon.get_visible_diameter())
			}
		}
		None => println!("{} does not have any moons", earth.name),
	}
}
