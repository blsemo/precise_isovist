use std::{error::Error, fmt::{Display, Formatter}};
use crate::rays::types::{Line, Point, LineConstructor};
use dxf::entities::{Entity, EntityType};

#[derive(Debug)]
pub struct ConversionError;

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conversion failed")
    }
}

impl Error for ConversionError {

}

fn point_to_dxf( p: &Point ) -> dxf::Point {
    dxf::Point::new(p.x, p.y, 0.0)
}

fn point_from_dxf( p : &dxf::Point) -> Point {
    Point::new(p.x, p.y)
}
pub trait DxfConverter<T> {
    fn from_dxf(entity: &Entity) -> Result<T, ConversionError>;
    fn to_dxf(self: &Self) -> Entity;
}

impl DxfConverter<Line> for Line {
    fn from_dxf( entity : &Entity ) -> Result<Line, ConversionError> {
        match entity.specific {
            EntityType::Line( ref l) => {
                return Ok(Line::from_points(&point_from_dxf(&l.p1), &point_from_dxf(&l.p2)));
            }
            _ =>  {
                return Err(ConversionError);
            }
        }
    }

    fn to_dxf(self: &Self) -> Entity {
        return Entity::new(EntityType::Line(dxf::entities::Line::new(point_to_dxf(&self.a), point_to_dxf(&self.b))))
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_line_conversion(){
        let dxf_line = Entity::new(dxf::entities::EntityType::Line(dxf::entities::Line::new(dxf::Point::new(0.0, 0.0, 0.0), dxf::Point::new(0.0,1.0, 0.0))));
        let line = Line::from_dxf(&dxf_line).expect("Conversion failed");
        assert!( line.a.x == 0.0 );
        assert!( line.a.y == 0.0 );
        assert!( line.b.x == 0.0 );
        assert!( line.b.y == 1.0 );

        let re_dxf_line = line.to_dxf();

        match re_dxf_line.specific {
            EntityType::Line( ref l) => {
                assert_eq!(l.p1.x, 0.0);
                assert_eq!(l.p1.y, 0.0);
                assert_eq!(l.p2.x, 0.0);
                assert_eq!(l.p2.y, 1.0);
            }
            _ => {
                assert!(false, "Entity is not a line");
            }
        }
    }
}