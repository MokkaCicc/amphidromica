mod enums;
mod helpers;
mod structs;
mod traits;

use enums::Direction;
use helpers::{Period, Time};
use structs::{Moon, Planet};
use traits::Body;

fn main() {
	let mut earth = Planet::new(
		"Earth",
		6.371e6,
		5.973e24,
		Period::new(Time::new(1, 0, 0, 0), Direction::Prograde),
	);
	let sun = Moon::new(
		"Sun",
		6.963e8,
		1.989e30,
		1.496e11,
		// Retrograde because assimiled to a moon
		Direction::Retrograde,
		(earth.radius(), earth.mass()),
	);
	let moon = Moon::new(
		"Pomia",
		1.737e6,
		7.348e22,
		3.844e8,
		Direction::Prograde,
		(earth.radius(), earth.mass()),
	);

	earth.add_moon(&sun);
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
				println!("\tdistance : {:10.3e}m", moon.distance);
				println!("\tinfluence : {:.3}", moon.tidal_influence / 2.0);
				println!("\torbital period : {}", moon.orbital_period.time);
				println!("\tsynodic period : {}", moon.get_synodic_period(&earth));
				println!("\tvisible diameter : {:.3}cm", moon.get_visible_diameter())
			}
		}
		None => println!("{} does not have any moons", earth.name()),
	}
}
