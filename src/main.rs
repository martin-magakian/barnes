use std::thread;

static N_THREAD: i32 = 5;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub name:&'static str,
}


impl Point{
	pub fn new(x: i32, y: i32, name:&'static str) -> Point{
		Point { x:x, y:y, name:&name }
	}
}

#[derive(Debug)]
pub struct Region{
	pub nw: Square,
	pub ne: Square,
	pub sw: Square,
	pub se: Square
}

impl Region {
	pub fn new(nw: Square, ne:Square, sw:Square, se:Square) -> Region {
		Region{ nw:nw, ne:ne, sw:sw, se:se }
	}
}


#[derive(Debug)]
pub struct Square {
    pub x: i32,
    pub y: i32,
	pub lenght: i32,
	pub weight: i32,
	pub point: Option<Point>,
	pub bucket: Option<Vec<Point>>,
	pub region: Option<Box<Region>>
}

impl Square {

	pub fn new(x:i32, y:i32, lenght:i32) -> Square {
		Square { x:x, y:y, lenght:lenght, weight: 0, point: None, bucket: None, region: None}
	}

    pub fn is_inside(&self, point: &Point) -> bool {
		if point.x >= self.x && point.x <= self.x+self.lenght { // ok x
			if point.y >= self.y && point.y <= self.y+self.lenght { // ok y
				return true;
			}
		}
    	false
    }
    
    /*fn fill_bucket(&mut self, mother_bucket: Vec<Point>) -> (Vec<Point>, Vec<Point>){
		
		let mut inside = vec![];
		let mut outside = vec![];
		for p in mother_bucket{
			if self.is_inside(&p) {
				inside.push(p);
			}else{
				outside.push(p);
			}
		}
		(inside , outside)
    }
    
    pub fn compute_bucket(&self, bucket: Vec<Point>) -> (Option<Point>, Option<Region>){
		match self.weight{
			0 => {
				(None, None)
			}
			1 =>  {
				let point:Point = bucket.pop().unwrap();
				(Some(point), None)
			},
			_ => {
				let region = self.split_fill(bucket);
				(None, Some(region))
			}
		}
    }
    
    pub fn split(&self) -> (Square, Square, Square, Square) {
    	let nw = Square::new(self.x, self.y + self.lenght / 2, self.lenght / 2);
    	let ne = Square::new(self.x + self.lenght / 2, self.y + self.lenght / 2, self.lenght / 2);
    	let se = Square::new(self.x + self.lenght / 2, self.y, self.lenght / 2);
    	let sw = Square::new(self.x, self.y, self.lenght / 2);
    	(nw, ne , sw, se)
    }
    

    
    fn split_fill(&mut self, mut current_bucket: Vec<Point>) -> Region {
    	let (mut nw, mut ne, mut sw, mut se) = self.split();
    	
		let (nw_bucket, current_bucket) = nw.fill_bucket(current_bucket);
		let (ne_bucket, current_bucket) = ne.fill_bucket(current_bucket);
		let (sw_bucket, current_bucket) = sw.fill_bucket(current_bucket);
		let (se_bucket, current_bucket) = se.fill_bucket(current_bucket);
		
		let (point, region) = nw.compute_bucket(nw_bucket);
		self.assign(&mut nw, point, region);
		let (point, region) = ne.compute_bucket(ne_bucket);
		self.assign(&mut ne, point, region);
		let (point, region) = sw.compute_bucket(sw_bucket);
		self.assign(&mut sw, point, region);
		let (point, region) = se.compute_bucket(se_bucket);
		self.assign(&mut se, point, region);
		
		Region::new(nw, ne, sw, se)
    }
    
    pub fn compute(&mut self, root_bucket :Vec<Point>){
		self.weight = root_bucket.len() as i32;
		self.split_fill(root_bucket);
	}*/
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

fn compute(mut self_square :Square, mut bucket:Vec<Point>) -> (Square, Option<Point>, Option<Region>){
	self_square.weight = bucket.len() as i32;
	if bucket.len() == 0 {
		return (self_square, None, None)
	}
	if bucket.len() == 1 {
		let point = bucket.pop().unwrap();
		return (self_square, Some(point), None)
	}
	
	let (mut nw, mut ne, mut sw, mut se) = split_region(&self_square);

	let (nw_bucket, current_bucket) = use_bucket(&mut nw, bucket);
	let (ne_bucket, current_bucket) = use_bucket(&mut ne, current_bucket);
	let (sw_bucket, current_bucket) = use_bucket(&mut sw, current_bucket);
	let (se_bucket, _) = use_bucket(&mut se, current_bucket);

	let (nw, nw_point, nw_region) = compute(nw, nw_bucket);
	let (ne, ne_point, ne_region) = compute(ne, ne_bucket);
	let (sw, sw_point, sw_region) = compute(sw, sw_bucket);
	let (se, se_point, se_region) = compute(se, se_bucket);
	
	let nw = assign(nw, nw_point, nw_region);
	let ne = assign(ne, ne_point, ne_region);
	let sw = assign(sw, sw_point, sw_region);
	let se = assign(se, se_point, se_region);
	
	let region = Region::new(nw, ne, sw, se);
	self_square.region = Some(Box::new(region));

	(self_square, None, None)
}

pub fn start(mut square: Square, root_bucket :Vec<Point>) -> Square {
	square.weight = root_bucket.len() as i32;
	let (square, _, _) = compute(square, root_bucket);
	square
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

fn main() {
	let mut square = Square::new(0, 0, 80);
	square = start(square, create_points());

	println!("{:#?}", square);
}