Barnes-Hut tree in Rust
============
Based on [The Barnes-Hut Algorithm](http://arborjs.org/docs/barnes-hut) by Tom Ventimiglia & Kevin Wayne.<br />
"A quad-tree is similar to a binary tree, except that each node has 4 children"


Made to learn Rust
---------
My friend [Tristan Brismontier](https://github.com/TristanBrismontier) was building a (more advance) [Barnes-Hut in C# using Unity](https://github.com/TristanBrismontier/Barnes-Hut-Algorithm).<br />
It looks like a nice project for learning Rust.<br />
Also, an interesting candidate for building an [Anomaly Detection Service](https://anomaly.io)


How to use
---------

Add barnes dependency in Cargo.toml:
```Rust
[dependencies]

barnes = "0.1.0"
```


```Rust
extern crate barnes;
use data::{Point, Square, Region};
use tree::Tree;

fn create_points() -> Vec<Point> {
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
	square = Tree.compute_root(square, create_points());
	
	println!("{:?}", square);
}
```

This code use 8 points:<br />
![barnes-hut quadrant](https://raw.github.com/martin-magakian/Barnes-Hut/master/README_src/quadrant.png)

It produce this quadtree:<br />
![barnes-hut tree](https://raw.github.com/martin-magakian/Barnes-Hut/master/README_src/tree.png)

<br />
The display:<br />
```JS
Square {
    x: 0,
    y: 0,
    lenght: 80,
    weight: 8,
    point: None,
    region: Some(
        Region {
            nw: Square {
                x: 0,
                y: 40,
                lenght: 40,
                weight: 1,
                point: Some(
                    Point {
                        x: 13,
                        y: 62,
                        name: "A"
                    }
                ),
                region: None
            },
            ne: Square {
                x: 40,
                y: 40,
                lenght: 40,
                weight: 3,
                point: None,
                region: Some(
                    Region {
                        nw: Square {
                            x: 40,
                            y: 60,
                            lenght: 20,
                            weight: 2,
                            point: None,
                            region: Some(
                                Region {
                                    nw: Square {
                                        x: 40,
                                        y: 70,
                                        lenght: 10,
                                        weight: 0,
                                        point: None,
                                        region: None
                                    },
                                    ne: Square {
                                        x: 50,
                                        y: 70,
                                        lenght: 10,
                                        weight: 1,
                                        point: Some(
                                            Point {
                                                x: 54,
                                                y: 72,
                                                name: "B"
                                            }
                                        ),
                                        region: None
                                    },
                                    sw: Square {
                                        x: 40,
                                        y: 60,
                                        lenght: 10,
                                        weight: 1,
                                        point: Some(
                                            Point {
                                                x: 45,
                                                y: 65,
                                                name: "C"
                                            }
                                        ),
                                        region: None
                                    },
                                    se: Square {
                                        x: 50,
                                        y: 60,
                                        lenght: 10,
                                        weight: 0,
                                        point: None,
                                        region: None
                                    }
                                }
                            )
                        },
                        ne: Square {
                            x: 60,
                            y: 60,
                            lenght: 20,
                            weight: 0,
                            point: None,
                            region: None
                        },
                        sw: Square {
                            x: 40,
                            y: 40,
                            lenght: 20,
                            weight: 0,
                            point: None,
                            region: None
                        },
                        se: Square {
                            x: 60,
                            y: 40,
                            lenght: 20,
                            weight: 1,
                            point: Some(
                                Point {
                                    x: 62,
                                    y: 57,
                                    name: "D"
                                }
                            ),
                            region: None
                        }
                    }
                )
            },
            sw: Square {
                x: 0,
                y: 0,
                lenght: 40,
                weight: 3,
                point: None,
                region: Some(
                    Region {
                        nw: Square {
                            x: 0,
                            y: 20,
                            lenght: 20,
                            weight: 0,
                            point: None,
                            region: None
                        },
                        ne: Square {
                            x: 20,
                            y: 20,
                            lenght: 20,
                            weight: 1,
                            point: Some(
                                Point {
                                    x: 38,
                                    y: 38,
                                    name: "E"
                                }
                            ),
                            region: None
                        },
                        sw: Square {
                            x: 0,
                            y: 0,
                            lenght: 20,
                            weight: 1,
                            point: Some(
                                Point {
                                    x: 11,
                                    y: 5,
                                    name: "F"
                                }
                            ),
                            region: None
                        },
                        se: Square {
                            x: 20,
                            y: 0,
                            lenght: 20,
                            weight: 1,
                            point: Some(
                                Point {
                                    x: 32,
                                    y: 11,
                                    name: "G"
                                }
                            ),
                            region: None
                        }
                    }
                )
            },
            se: Square {
                x: 40,
                y: 0,
                lenght: 40,
                weight: 1,
                point: Some(
                    Point {
                        x: 52,
                        y: 8,
                        name: "H"
                    }
                ),
                region: None
            }
        }
    )
}
```

Performance
-------

in x: number of point to place in the tree<br />
in y: time used in second


![perf1](https://raw.github.com/martin-magakian/Barnes-Hut/master/README_src/perf1.png)

28 second to insert 40.000.000 point in the Barnes-Hut tree.<br />
(on MacBook Pro 8 core)


Contact
=========
Developed by Martin Magakian dev.martin.magakian@gmail.com<br />
by [Anomaly Detection](https://anomaly.io)


License
=========
MIT License (MIT)
