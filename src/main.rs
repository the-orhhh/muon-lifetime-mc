use rand::Rng;

/// Generate a random number from cos^2(theta) distribution using inverse transform sampling
/// Returns angle in [0, π/2]
fn sample_cos2<R: Rng>(rng: &mut R) -> f64 {
    let u: f64 = rng.random_range(0.0..1.0);
    u.sqrt().acos()
}

/// Check if a ray from (x, y, 0) going in +z direction intersects the rotated cylinder
/// Cylinder: radius R, height H, rotated by angle theta around x-axis
/// After rotation: axis is in y-z plane at angle theta from z-axis
fn ray_intersects_rotated_cylinder(x: f64, y: f64, radius: f64, height: f64, theta: f64) -> bool {
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    
    // Cylinder axis direction vector: (0, sin(theta), cos(theta))
    // For ray point (x, y, t), closest point on axis has parameter s = y*sin(theta) + t*cos(theta)
    // Distance squared from ray point to axis: x² + (y*cos(theta) - t*sin(theta))²
    
    // Position along axis must be in range [-H/2, H/2]
    let t_min = (-height / 2.0 - y * sin_theta) / cos_theta;
    let t_max = (height / 2.0 - y * sin_theta) / cos_theta;
    
    if t_max < 0.0 {
        return false;
    }
    
    let t_start = t_min.max(0.0);
    if t_start > t_max {
        return false;
    }
    
    // Find minimum distance in the valid t range
    let t_opt = if sin_theta > 1e-10 {
        y * cos_theta / sin_theta
    } else {
        t_start
    };
    
    let t_check = t_opt.max(t_start).min(t_max);
    let dist_sq = x * x + (y * cos_theta - t_check * sin_theta).powi(2);
    
    dist_sq < radius * radius
}

/// Calculate effective area using Monte Carlo simulation
fn calculate_effective_area(radius: f64, height: f64, num_samples: usize, rng: &mut impl Rng) -> f64 {
    let theta = sample_cos2(rng);
    
    let max_extent_x = radius * 1.5;
    let max_extent_y = (radius + height * 0.5) * 1.5;
    let sampling_area = (2.0 * max_extent_x) * (2.0 * max_extent_y);
    
    let mut hits = 0;
    for _ in 0..num_samples {
        let x = rng.random_range(-max_extent_x..max_extent_x);
        let y = rng.random_range(-max_extent_y..max_extent_y);
        
        if ray_intersects_rotated_cylinder(x, y, radius, height, theta) {
            hits += 1;
        }
    }
    
    (hits as f64 / num_samples as f64) * sampling_area
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Error: Expected exactly 2 arguments (radius and height)");
        std::process::exit(1);
    }
    
    let radius: f64 = args[1].parse()
        .expect("Error: radius must be a valid number");
    let height: f64 = args[2].parse()
        .expect("Error: height must be a valid number");
    
    if radius <= 0.0 || height <= 0.0 {
        eprintln!("Error: radius and height must be positive");
        std::process::exit(1);
    }
    
    println!("Monte Carlo Simulation for Cylinder Effective Area");
    println!("Radius: {}, Height: {}", radius, height);
    
    let mut rng = rand::rng();
    let num_samples = 100_000;
    
    println!("Running {} Monte Carlo samples...", num_samples);
    
    let effective_area = calculate_effective_area(radius, height, num_samples, &mut rng);
    
    println!("\nResults:");
    println!("Effective Area: {:.6}", effective_area);
    println!("Geometric Area (πr²): {:.6}", std::f64::consts::PI * radius * radius);
}
