use std::sync::atomic::{AtomicU32, Ordering};
use std::hash::{Hash, Hasher};

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
}

impl Point {
    pub fn new( x: f64, y:f64 ) -> Point {
        Point{
            x: OrderedFloat::from(x),
            y: OrderedFloat::from(y),
        }
    }

    pub fn new_ordered( x: OrderedFloat<f64>, y: OrderedFloat<f64>) -> Point {
        Point{
            x: x,
            y: y,
        }
    }
}

pub trait LineConstructor { 
    fn from_points( a: &Point, b: &Point ) -> Line;
    fn from_coords( x1: f64, y1: f64, x2: f64, y2: f64) -> Line;
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq)]
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

impl PartialEq for Line{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Line{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
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
        assert!(line1.b.x == 0.0);
        assert!(line1.b.y == 2.0);

        let line2 = Line::from_coords(0.0, 1.0 , 1.5, 0.2);
        assert!(line2.a.x == 0.0);
        assert!(line2.a.y == 1.0);
        assert!(line2.b.x == 1.5);
        assert!(line2.b.y == 0.2);

        assert_eq!(line1.id() + 1, line2.id())
    }
}
