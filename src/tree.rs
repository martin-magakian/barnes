use data::{Point, Region, Square};
use std::thread;


static mut THREAD_LIMIT: i64 = 0;
fn find_thread_limit(bucket_size:i64) -> i64{
	let thread = 8;
	
	let limit = bucket_size / (thread * 10);
	limit
}

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


pub struct Tree;
impl Tree{
	
	pub fn compute_root(&self, mut square: Square, root_bucket :Vec<Point>) -> Square {
		let bucket_size = root_bucket.len() as i64;
		square.weight = bucket_size;
		unsafe {
			THREAD_LIMIT = find_thread_limit(bucket_size)
			}
		let (square, _, _) = compute(square, root_bucket);
		square
	}
}



