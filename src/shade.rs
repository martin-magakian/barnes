#[derive(Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub name:&'static str,
}


impl Point{
	pub fn new(x: i64, y: i64, name:&'static str) -> Point{
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
    pub x: i64,
    pub y: i64,
	pub lenght: i64,
	pub weight: i64,
	pub point: Option<Point>,
	pub region: Option<Box<Region>>
}

impl Square {

	pub fn new(x:i64, y:i64, lenght:i64) -> Square {
		Square { x:x, y:y, lenght:lenght, weight: 0, point: None, region: None}
	}

    fn is_inside(&self, point: &Point) -> bool {
		if point.x >= self.x && point.x <= self.x+self.lenght { // ok x
			if point.y >= self.y && point.y <= self.y+self.lenght { // ok y
				return true;
			}
		}
    	false
    }
    
    fn fill_bucket(&mut self,  mother_bucket: Vec<Point>) -> Vec<Point>{
		
		let mut eligable = vec![];
		let mut not_used = vec![];
		for p in mother_bucket{
			if self.is_inside(&p) {
				eligable.push(p);
			}else{
				not_used.push(p);
			}
		}
		
		self.weight = eligable.len() as i64;
		match self.weight{
			0 => {
				not_used
			}
			1 =>  {
				self.point = Some(eligable.pop().unwrap());
				not_used
			},
			_ => {
				let left_over = self.split_fill(eligable);
				for e in left_over {
					not_used.push(e);
				}
				not_used
			}
		}
    }
    
    fn split(&self) -> Region {
    	let nw = Square::new(self.x, self.y + self.lenght / 2, self.lenght / 2);
    	let ne = Square::new(self.x + self.lenght / 2, self.y + self.lenght / 2, self.lenght / 2);
    	let se = Square::new(self.x + self.lenght / 2, self.y, self.lenght / 2);
    	let sw = Square::new(self.x, self.y, self.lenght / 2);
    	Region::new(nw, ne , sw, se)
    }
    
    
    fn split_fill(&mut self, mut bucket: Vec<Point>) -> Vec<Point> {
    	let mut region = self.split();
    	
		bucket = region.nw.fill_bucket(bucket);
		bucket = region.ne.fill_bucket(bucket);
		bucket = region.sw.fill_bucket(bucket);
		bucket = region.se.fill_bucket(bucket);
		
		self.region = Some(Box::new(region));
		bucket
    }
    
    pub fn compute(&mut self, bucket :Vec<Point>){
		self.weight = bucket.len() as i64;
		self.split_fill(bucket);
	}
}