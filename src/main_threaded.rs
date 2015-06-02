

use shade::Point;
use shade::Square;
use shade::Region;
use std::thread;



fn main() {
	let mut square = Square::new(0, 0, 80);
	let (mut nw, mut ne, mut sw, mut se) = square.split();
	
	let mut nw_bucket = vec![];
	let mut ne_bucket = vec![];
	let mut sw_bucket = vec![];
	let mut se_bucket = vec![];
	for p in points {
		if nw.is_inside(&p) {
			nw_bucket.push(p);
		}else if ne.is_inside(&p) {
			ne_bucket.push(p);
		}else if sw.is_inside(&p) {
			sw_bucket.push(p);
		}else{
			se_bucket.push(p);
		}
	}
	
	let nw_t = thread::spawn(move || {
	    nw.compute(nw_bucket);
	    nw
	});
	let ne_t = thread::spawn(move || {
	    ne.compute(ne_bucket);
	    ne
	});
	let sw_t = thread::spawn(move || {
	    sw.compute(sw_bucket);
	    sw
	});
	let se_t = thread::spawn(move || {
	    se.compute(se_bucket);
	    se
	});
	
	
	let region:Region = Region::new(nw_t.join().unwrap(), ne_t.join().unwrap(), sw_t.join().unwrap(), se_t.join().unwrap());
	square.region = Some(Box::new(region));

	println!("{:#?}", square);
}