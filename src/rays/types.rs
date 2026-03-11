use ordered_float::OrderedFloat;

#[derive(Debug, Copy)]
#[derive(Clone)]
#[derive(Eq,PartialEq,Hash)]
pub struct Point {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>
}

impl Point {
    pub fn new( x: f64, y:f64 ) -> Point {
        Point{
            x: OrderedFloat::from(x),
            y: OrderedFloat::from(y)
        }
    }
}

pub trait LineConstructor { 
    fn from_points( a: &Point, b: &Point ) -> Line;
    fn from_coords( x1: f64, y1: f64, x2: f64, y2: f64) -> Line;
}

#[derive(Debug)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}

impl LineConstructor for Line{
    fn from_points( a: &Point, b: &Point ) -> Line {
        Line { a: a.clone(), b: b.clone() }
    }

    fn from_coords( x1: f64, y1: f64, x2: f64, y2: f64 ) -> Line {
        Line{ a: Point::new( x1, y1 ), b: Point::new( x2, y2 )}
    }
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn test_line_conversion(){
        let a = Point::new(1.0, 1.0);
        let b = Point::new(0.0, 2.0);

        let line1 = Line::from_points(&a, &b);

        // check a is still valid!
        assert!(a.x == 1.0);

        // check the line
        assert!(line1.a.x == 1.0);
        assert!(line1.a.y == 1.0);
        assert!(line1.b.x == 0.0);
        assert!(line1.b.y == 2.0);

        let line2 = Line::from_coords(0.0, 1.0 , 1.5, 0.2);
        assert!(line2.a.x == 0.0);
        assert!(line2.a.y == 1.0);
        assert!(line2.b.x == 1.5);
        assert!(line2.b.y == 0.2);
    }
}
