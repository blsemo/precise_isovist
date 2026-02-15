use std::{fmt::Debug};

use super::types::*;

#[derive(Debug)]
pub struct Intersection{
    pub point: Point,
    pub scale_factor: f64
}
#[derive(Debug)]
pub struct CalcError{
    pub msg: String,
}

impl CalcError {
    fn new( msg: &str ) -> CalcError {
        CalcError{ msg: msg.to_string()}
    }
}

const EPSILON: f64 = 0.000001;

fn section_point( ray: &Line, line: &Line ) -> Result<Intersection, CalcError> {

    let d = (ray.b.x - ray.a.x) * (line.b.y - line.b.y) - (line.b.x - line.a.x) * (ray.b.y - ray.a.y);
    let r = ((line.b.x - line.a.x) * (ray.a.y - line.a.y) - (ray.a.x - line.a.x) * (line.b.y - line.a.y)) / d;
    if (r + EPSILON) < 0.0 { 
        return Err(CalcError::new("Ray pointing away from Element"));
    }

    let s = ((line.a.x - ray.a.x) * (ray.b.y - ray.a.y) - (ray.b.x - ray.a.x) * (line.a.y - ray.a.y)) / d;
    if (s + EPSILON) < 0.0 || (s - EPSILON) > 1.0 {
        return Err(CalcError::new("Ray missing Element"));
    }

  return Ok(Intersection {     
    point: Point{   
  	x: s * (line.b.x - line.a.x) + line.a.x, 
    y: s * (line.b.y - line.a.y) + line.a.y,
    },
    scale_factor: r,
  });
}

pub fn closest_section( ray: &Line, lines: &Vec<Line>) -> Option<Intersection> {
    let mut intersection: Option<Intersection> = None;
    
    for line in lines {
        let i = section_point(ray, line);
        if i.is_ok() {
            let iv = i.unwrap();
            if intersection.is_none() ||  iv.scale_factor < intersection.as_ref().unwrap().scale_factor {
                intersection = Some(iv);
            }
        }
    };

    return intersection
}

#[cfg(test)]


mod tests {
use super::*;

    #[test]
    fn test_section_point(){
        let l = Line{ a: Point{ x: 1.0, y: 1.0}, b: Point{x: 0.0, y: 1.0} };
        let r = Line{ a: Point{ x: 0.5, y: 0.0}, b: Point{x: 0.5, y: 1.5} };

        let intersection = section_point(&r, &l).expect("Calculation failed");
        assert!(intersection.point.x == 0.5);
        assert!(intersection.point.y == 1.0);
        println!("Distance: {}", intersection.scale_factor);
        assert!(intersection.scale_factor > 2.0/3.0 - EPSILON || intersection.scale_factor > 2.0/3.0 + EPSILON);
    }

    #[test]
    fn test_invalid_sections(){
        let l = Line{ a: Point{ x: 1.0, y: 1.0}, b: Point{x: 0.0, y: 1.0} };
        let r = Line{ a: Point{ x: 0.5, y: 0.5}, b: Point{x: 0.5, y: 0.0} };

        let error = section_point(&r, &l).expect_err("Fail - the calculation worked!");
        assert!(error.msg == "Ray pointing away from Element");

        let l2 = Line::from_coords(0.0, -1.0, 0.3, -1.0);
        let error = section_point(&r, &l2).expect_err("Fail - the calculation worked!");
        assert!(error.msg == "Ray missing Element");


    }

    #[test]
    fn find_closest_section(){
        let lines = vec!(
            Line::from_coords(0.0, 1.0, 1.0, 1.0),
            Line::from_coords(0.0, 1.5, 1.0, 2.0)
        );

        let r1 = Line::from_coords(0.5, 0.0, 0.5, 0.5);

        let i = closest_section(&r1, &lines).expect("No point returned!");
        assert!(i.point.x == 0.5);
        println!("Intersection point {},{}", i.point.x, i.point.y);
        assert!(i.point.y == 1.0);

        let r2 = Line::from_coords(0.5, 0.0, 10.0, 0.2);

        assert!(closest_section(&r2, &lines).is_none());
    }
}
