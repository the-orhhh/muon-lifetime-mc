use rand::Rng;

/// Custom distribution for generating numbers
/// Uses inverse transform sampling for O(1) performance
pub struct CustomDistribution {
    // Parameters for your distribution
}

impl CustomDistribution {
    pub fn new() -> Self {
        // For a cos^2(theta) distribution over theta in [0, pi/2],
        // no parameters are needed, so we can remove theta.
        CustomDistribution { }
    }

    /// Generate a random number from the custom distribution
    pub fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        let u: f64 = rng.random();
        // Inverse transform: acos(sqrt(u)) for cos^2(theta) distribution
        u.sqrt().acos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rng;

    #[test]
    fn test_sample() {
        let dist = CustomDistribution::new();
        let mut rng = rng();
        let value = dist.sample(&mut rng);
        assert!(value >= 0.0);
    }
}