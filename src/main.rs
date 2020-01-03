struct Qubit {
    theta: f64,
    phi: f64
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_qubit() {
	let qubit = Qubit{};
	assert_eq!(qubit.theta, 0.0);
	assert_eq!(qubit.phi, 0.0);
    }
}
