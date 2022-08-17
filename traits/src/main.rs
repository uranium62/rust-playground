use std::fmt;
use std::fmt::Display;

struct Point {
    x: f64,
    y: f64,
}

fn point_to_string(p: &Point) -> String {
    format!("({},{})", p.x, p.y)
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };

    println!("{}", point_to_string(&p));
    println!("{}", p)
}
