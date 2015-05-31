extern crate time;
use time::{Duration, PreciseTime, SteadyTime};

extern crate rand;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

fn random_point(num_point: i64, max: i64) -> Vec<Point> {
	let mut rng = rand::thread_rng();
	let between = Range::new(0i64, max);
	
	(0..num_point).map(|e| {
						Point::new(between.ind_sample(&mut rng), between.ind_sample(&mut rng), "hey")
					}).collect()
}

fn run_benchmark(num_point: i64) {
	
	let grid_size = num_point*100;
	let rand_points = random_point(num_point, grid_size);
	
	let mut square = Square::new(0, 0, grid_size);
	square.weight = rand_points.len() as i64;
	
	println!("[start Barnes Hut]");
	let start = SteadyTime::now();
	square.compute(rand_points);
	let end = SteadyTime::now();
	println!("[/end Barnes Hut]");
	println!("total time: {}", end-start);
}

fn main() {
	let mut square = Square::new(0, 0, 80);
	square.compute(create_points());

	println!("nw = {:#?}", square);
}