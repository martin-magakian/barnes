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
	pub bucket: Option<Vec<Point>>,
	pub region: Option<Box<Region>>
}

impl Square {

	pub fn new(x:i64, y:i64, lenght:i64) -> Square {
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
}