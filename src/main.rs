mod enums;
mod helpers;
mod structs;
mod traits;

use enums::Direction;
use helpers::{Orbit, Period, Revolution};
use structs::{Moon, Planet, Sun};

fn main() {
	let mut sun = Sun::new(
		"Sun",
		6.963e8,
		1.989e30,
		Revolution::new(Period::new(27, 6, 43, 12), Direction::Prograde),
	);
	let mut earth = Planet::new(
		"Earth",
		6.371e6,
		5.973e24,
		Revolution::new(Period::new(1, 0, 0, 0), Direction::Prograde),
		Orbit::from_kepler_law(5.973e24 + sun.mass, 1.496e11, Direction::Prograde),
	);
	let moon = Moon::new(
		"Moon",
		1.737e6,
		7.348e22,
		Revolution::new(Period::new(27, 7, 42, 14), Direction::Prograde),
		Orbit::from_kepler_law(7.348e22 + earth.mass, 3.844e8, Direction::Prograde),
	);

	sun.add_planet(&earth);
	earth.add_moon(&moon);

	let hours = 69.0;
	match earth.get_tidal_range() {
		Some(range) => {
			println!("{}", range);
			println!("Tidal at {} hours : {:.3}m", hours, earth.tidal_at(hours));
			println!();
			for moon in &earth.moons {
				println!("{} : ", moon.name);
				println!("\tradius : {:10.3e}m", moon.radius);
				println!("\tmass : {:10.3e}kg", moon.mass);
				println!("\tdistance : {:10.3e}m", moon.orbit.distance);
				println!("\tinfluence : {:.3}", moon.tidal_influence(&earth) / 2.0);
				println!("\torbital period : {}", moon.orbit.period);
				println!(
					"\tsynodic period : {}",
					earth.revolution.synodic_period(&moon.orbit)
				);
				println!("\tvisible diameter : {:.3}cm", moon.get_visible_diameter())
			}
		}
		None => println!("{} does not have any moons", earth.name),
	}
}
