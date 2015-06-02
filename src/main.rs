mod data;
use data::{Point, Square, Region};

use std::thread;

extern crate time;
use time::{Duration, PreciseTime, SteadyTime};

extern crate rand;
use rand::distributions::{IndependentSample, Range};
use rand::Rng;

static N_THREAD: i64 = 5;



pub fn use_bucket(square:&mut Square, mother_bucket: Vec<Point>) -> (Vec<Point>, Vec<Point>){
	let mut inside = vec![];
	let mut outside = vec![];
	for p in mother_bucket{
		if square.is_inside(&p) {
			inside.push(p);
		}else{
			outside.push(p);
		}
	}
	(inside , outside)
}

pub fn split_region(square :&Square) -> (Square, Square, Square, Square) {
	let nw = Square::new(square.x, square.y + square.lenght / 2, square.lenght / 2);
	let ne = Square::new(square.x + square.lenght / 2, square.y + square.lenght / 2, square.lenght / 2);
	let se = Square::new(square.x + square.lenght / 2, square.y, square.lenght / 2);
	let sw = Square::new(square.x, square.y, square.lenght / 2);
	(nw, ne , sw, se)
}

fn assign(mut current_square :Square, point:Option<Point>, region:Option<Region>) -> Square {
	if region.is_some() {
		current_square.region = Some(Box::new(region.unwrap()));
	}else{
		current_square.point = point;
	}
	current_square
}

fn compute_assign(square:Square, bucket :Vec<Point>) -> Square {
	let (square, nw_point, nw_region) = compute(square, bucket);
	let square = assign(square, nw_point, nw_region);
	square
}

fn create_region_no_thread(nw:Square,nw_bucket:Vec<Point>, ne:Square,ne_bucket:Vec<Point>, sw:Square,sw_bucket:Vec<Point>, se:Square,se_bucket:Vec<Point>) -> Region {
	let nw = compute_assign(nw, nw_bucket);
	let ne = compute_assign(ne, ne_bucket);
	let sw = compute_assign(sw, sw_bucket);
	let se = compute_assign(se, se_bucket);
	
	Region::new(nw, ne, sw, se)
}

fn create_region_thread(nw:Square,nw_bucket:Vec<Point>, ne:Square,ne_bucket:Vec<Point>, sw:Square,sw_bucket:Vec<Point>, se:Square,se_bucket:Vec<Point>) -> Region {
	
	let nw_t = thread::spawn(move || {
	    compute_assign(nw, nw_bucket)
	});
	let ne_t = thread::spawn(move || {
	    compute_assign(ne, ne_bucket)
	});
	let sw_t = thread::spawn(move || {
	    compute_assign(sw, sw_bucket)
	});
	let se_t = thread::spawn(move || {
	    compute_assign(se, se_bucket)
	});
	
	Region::new(nw_t.join().unwrap(),
		 ne_t.join().unwrap(),
		 sw_t.join().unwrap(),
		 se_t.join().unwrap())
}



fn create_region(self_square :&Square, bucket :Vec<Point>) -> Region {
	let size = bucket.len() as i64;
	
	let (mut nw, mut ne, mut sw, mut se) = split_region(&self_square);
	let (nw_bucket, current_bucket) = use_bucket(&mut nw, bucket);
	let (ne_bucket, current_bucket) = use_bucket(&mut ne, current_bucket);
	let (sw_bucket, current_bucket) = use_bucket(&mut sw, current_bucket);
	let (se_bucket, _) = use_bucket(&mut se, current_bucket);
	
	if size > unsafe{THREAD_LIMIT} {
		println!("4 thread");
		create_region_thread(nw,nw_bucket, ne,ne_bucket, sw,sw_bucket, se,se_bucket)
	}else{
		create_region_no_thread(nw,nw_bucket, ne,ne_bucket, sw,sw_bucket, se,se_bucket)
	}
}


fn compute(mut self_square :Square, mut bucket:Vec<Point>) -> (Square, Option<Point>, Option<Region>){
	self_square.weight = bucket.len() as i64;
	if bucket.len() == 0 {
		return (self_square, None, None)
	}
	if bucket.len() == 1 {
		let point = bucket.pop().unwrap();
		return (self_square, Some(point), None)
	}
	
	let region = create_region(&mut self_square, bucket);
	self_square.region = Some(Box::new(region));

	(self_square, None, None)
}

fn find_thread_worth_ratio(bucket_size:i64) -> i64{
	let thread = 8;
	
	let limit = bucket_size / (thread * 10);
	limit
}

static mut THREAD_LIMIT: i64 = 0;
pub fn start(mut square: Square, root_bucket :Vec<Point>) -> Square {
	let bucket_size = root_bucket.len() as i64;
	square.weight = bucket_size;
	unsafe {
		THREAD_LIMIT = find_thread_worth_ratio(bucket_size)
		}
	let (square, _, _) = compute(square, root_bucket);
	square
}





///////MAIN 





fn random_point(num_point: i64, max: i64) -> Vec<Point> {
	let mut rng = rand::thread_rng();
	let between = Range::new(0i64, max);
	
	
	(0..num_point).map(|e| {
						Point::new(between.ind_sample(&mut rng), between.ind_sample(&mut rng), "hey")
					}).collect()
}





fn create_points() -> Vec<Point>{
	vec![
		Point::new(13, 62, "A"),
		Point::new(45, 65, "C"),
		Point::new(54, 72, "B"),
		Point::new(62, 57, "D"),
		Point::new(38, 38, "E"),
		Point::new(11, 5, "F"),
		Point::new(32, 11, "G"),
		Point::new(52, 8, "H"),
		]
}

/*fn main() {
	let mut square = Square::new(0, 0, 80);
	square = start(square, create_points());

	println!("{:#?}", square);
}*/


fn run_benchmark(num_point: i64) {
	
	let grid_size = num_point*100;
	println!("[mono-thread] - generate random point");
	let startTimer = SteadyTime::now();
	let rand_points = random_point(num_point, grid_size);
	println!("[/end] - {}", SteadyTime::now()-startTimer);
	
	println!("[multi-thread] - start Barnes Hut");
	let startTimer = SteadyTime::now();
	let mut square = Square::new(0, 0, grid_size);
	square = start(square, rand_points);
	println!("[/end] - {}", SteadyTime::now()-startTimer);
	
}



fn main() {
	run_benchmark(20_000_000);
}
