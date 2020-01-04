extern crate rand;

use rand::Rng;
use std::f64;

struct Qubit {
    theta: f64,
    phi: f64
}

impl Default for Qubit {
    fn default() -> Self {
	return Qubit{ theta: 0.0, phi: 0.0 };
    }	
}

impl Qubit {
    fn measure(qubit: &Qubit) -> i32 {
	let mut rng = rand::thread_rng();
	let rn = rng.gen::<f64>();

	if rn < (qubit.theta / 2.0).cos().powf(2.0) {
	    return 0
	}
	return 1
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_qubit() {
	let qubit: Qubit = Default::default();
	assert_eq!(qubit.theta, 0.0);
	assert_eq!(qubit.phi, 0.0);
    }

    #[test]
    fn test_neasure_default_qubit() {
	let qubit: Qubit = Default::default();

	let want = 0;
	let got = Qubit::measure(&qubit);
	assert_eq!(got, want);
    }

    #[test]
    fn test_measure_configured_as_one_qubit() {
	let qubit = Qubit{ theta: f64::consts::PI, phi: 0.0 };

	let want = 1;
	let got = Qubit::measure(&qubit);
	assert_eq!(got, want);
    }

    #[test]
    fn test_collapes_of_state() {
	let qubit = Qubit{ theta: f64::consts::PI / 2.0, phi: 0.0 };

	let want = Qubit::measure(&qubit);

	for n in 0..100 {
	    let got = Qubit::measure(&qubit);
	    assert_eq!(got, want);
	}
    }
}
