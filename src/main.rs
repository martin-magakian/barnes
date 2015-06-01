mod shade;
use shade::Point;
use shade::Square;



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
	square.compute(create_points());

	println!("{:#?}", square);
}
