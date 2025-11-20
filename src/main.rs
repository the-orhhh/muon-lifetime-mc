use std::env;

mod dist;
mod measure;

fn main() {
    let args: Vec<f64> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Error: Expected exactly 2 arguments (radius and height)");
        std::process::exit(1);
    }
    
    let radius: f64 = args[1].parse()
        .expect("Error: radius must be a valid number");
    let height: f64 = args[2].parse()
        .expect("Error: height must be a valid number");
    
    println!("Radius: {}, Height: {}", radius, height);
}
