use data::{Point, Square, Region};
use tree::Tree;

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

fn check_root(root:Region) -> Region {
	assert!(root.nw.point.is_some());
	assert!(root.nw.region.is_none());
	assert!(root.ne.point.is_none());
	assert!(root.ne.region.is_some());
	assert!(root.sw.point.is_none());
	assert!(root.sw.region.is_some());
	assert!(root.se.point.is_some());
	assert!(root.se.region.is_none());
	root
}

fn check_ne(ne:Region) {
	assert!(ne.nw.point.is_none());
	assert!(ne.nw.region.is_some());
	assert!(ne.ne.point.is_none());
	assert!(ne.ne.region.is_none());
	assert!(ne.sw.point.is_none());
	assert!(ne.sw.region.is_none());
	assert!(ne.se.point.is_some());
	assert!(ne.se.region.is_none());
	
	assert!(ne.ne.point.is_none());
	assert!(ne.ne.region.is_none());

	assert!(ne.sw.point.is_none());
	assert!(ne.sw.region.is_none());
	
	assert!(ne.se.point.is_some());
	assert!(ne.se.region.is_none());
}

fn check_sw(sw:Region) {
	assert!(sw.nw.point.is_none());
	assert!(sw.nw.region.is_none());
	assert!(sw.ne.point.is_some());
	assert!(sw.ne.region.is_none());
	assert!(sw.sw.point.is_some());
	assert!(sw.sw.region.is_none());
	assert!(sw.se.point.is_some());
	assert!(sw.se.region.is_none());
}



#[test]
fn demo() {
	println!("build tree from http://arborjs.org/docs/barnes-hut" );	
	let mut square = Square::new(0, 0, 80);
	square = Tree.compute_root(square, create_points());
	assert_eq!(square.weight, 8);
	assert!(square.region.is_some());
	
	let mut root:Region = *square.region.unwrap();
	assert_eq!(8, square.weight);
	assert_eq!(1, root.nw.weight);
	assert_eq!(3, root.ne.weight);
	assert_eq!(3, root.sw.weight);
	assert_eq!(1, root.se.weight);
	root = check_root(root);

	let ne = *root.ne.region.unwrap();
	check_ne(ne);

	let sw = *root.sw.region.unwrap();
	check_sw(sw);
}