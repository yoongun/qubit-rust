struct Qubit {
    theta: f64,
    phi: f64
}

impl Default for Qubit {
    fn default() -> Self {
	return Qubit{theta: 0.0, phi: 0.0};
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

	want = 0
	got = qubit.measure();
	assert_eq!(got, want);
    }

    #[test]
    fn test_measure_configured_qubit() {

    }
}
