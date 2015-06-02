use shade::Point;
use shade::Square;


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
	let me1 = rand_points.clone();
	me1.push(Point::new());
	let me2 = rand_points.clone();
	let me3 = rand_points.clone();
	let me4 = rand_points.clone();
	let me5 = rand_points.clone();
	println!("{}"
	
	square.compute(rand_points);
	let end = SteadyTime::now();
	println!("[/end Barnes Hut]");
	println!("total time: {}", end-start);
}

fn main() {
	run_benchmark(1_000_000);
}