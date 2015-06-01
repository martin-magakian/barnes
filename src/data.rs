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