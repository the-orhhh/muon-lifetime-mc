use rand::Rng;

/// Uniform distribution for generating uniform random measurements
pub struct UniformMeasure {
}

impl UniformMeasure {
    pub fn new(theta: f64) -> Self {
        UniformMeasure { }
    }

    /// Generate 100 pairs of uniform random numbers in [0, 1)
    pub fn sample<R: Rng>(&self, rng: &mut R) -> Vec<(f64, f64)> {
        (0..100).map(|_| (rng.gen(), rng.gen())).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_sample() {
        let measure = UniformMeasure::new(0.0);
        let mut rng = thread_rng();
        let pairs = measure.sample(&mut rng);
        assert_eq!(pairs.len(), 100);
        for (x, y) in pairs {
            assert!(x >= 0.0 && x < 1.0);
            assert!(y >= 0.0 && y < 1.0);
        }
    }
}