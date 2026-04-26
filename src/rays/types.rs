use std::sync::atomic::{AtomicU32, Ordering};

use ordered_float::OrderedFloat;

fn get_id() -> u32 {
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub trait Entity {
    fn id(self: &Self) -> u32;
}

#[derive(Debug, Copy)]
#[derive(Clone)]
#[derive(Eq,PartialEq,Hash)]
pub struct Point {
    pub x: OrderedFloat<f64>,
    pub y: OrderedFloat<f64>,
    id: u32,
}

impl Point {
    pub fn new( x: f64, y:f64 ) -> Point {
        Point{
            x: OrderedFloat::from(x),
            y: OrderedFloat::from(y),
            id: get_id(),
        }
    }

    pub fn new_ordered( x: OrderedFloat<f64>, y: OrderedFloat<f64>) -> Point {
        Point{
            x: x,
            y: y,
            id: get_id()
        }
    }
}

impl Entity for Point {
    fn id(self: &Self) -> u32 {
        self.id
    }
}

pub trait LineConstructor { 
    fn from_points( a: &Point, b: &Point ) -> Line;
    fn from_coords( x1: f64, y1: f64, x2: f64, y2: f64) -> Line;
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Line {
    pub a: Point,
    pub b: Point,
    id: u32,
}

impl LineConstructor for Line{
    fn from_points( a: &Point, b: &Point ) -> Line {
        Line { a: a.clone(), b: b.clone(), id: get_id() }
    }

    fn from_coords( x1: f64, y1: f64, x2: f64, y2: f64 ) -> Line {
        Line{ a: Point::new( x1, y1 ), b: Point::new( x2, y2 ), id: get_id()}
    }
}

impl Entity for Line {
    fn id(self: &Self) -> u32 {
        self.id
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
        assert_eq!(a.id(), line1.a.id());
        assert!(line1.b.x == 0.0);
        assert!(line1.b.y == 2.0);
        assert_eq!(b.id(), line1.b.id());

        let line2 = Line::from_coords(0.0, 1.0 , 1.5, 0.2);
        assert!(line2.a.x == 0.0);
        assert!(line2.a.y == 1.0);
        assert!(line2.b.x == 1.5);
        assert!(line2.b.y == 0.2);

        assert_eq!(line1.id() + 3, line2.id())
    }
}
