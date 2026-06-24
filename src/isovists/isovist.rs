use std::{error::Error, fmt::{Display}};
use crate::rays::{sections::{closest_intersections_to_all_vertices, sort_intersections, Intersection}, types::{Line, LineConstructor, Point}};

pub struct Border {
    pub line: Line,
    pub is_bounded: bool
}

struct InternalBorder<'l> {
    pub line: Line,
    pub on_line: Option<&'l Line>,
}

#[derive(Debug)]
pub struct IsovistError{
    pub msg: String
}

impl IsovistError{
    fn new(msg: &str) -> IsovistError {
        IsovistError{ msg: msg.to_string() }
    }
}

impl Display for IsovistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl Error for IsovistError {

}


pub fn find_isovist( plan: &Vec<Line>, point: &Point) -> Result<Vec<Border>, IsovistError> {
    let intersections = closest_intersections_to_all_vertices(point, plan);
    let sorted_intersections = sort_intersections(point, &intersections);
    let mut previous_point: Option<&Intersection> = None;
    let mut candidate_borders: Vec<InternalBorder> = vec![];

    for point in sorted_intersections {
        if let Some(previous) = previous_point{
            let mut line_section = previous.lines.intersection(&point.lines);
            candidate_borders.push(
                InternalBorder { line: Line::from_points(&previous.point, &point.point), on_line: line_section.next().map(|v| *v) }
            );
            if let Some(_) = line_section.next(){
                return Err(IsovistError::new ("More than one line attached to a single border" ));
            }
        }
        previous_point = Some(point)
    }
    



    Ok(vec![])
}

#[cfg(test)]

mod tests{
    use super::*;


    fn create_square() -> Vec<Line> {
        vec![
            Line::from_coords(0.0, 0.0, 0.0, 1.0),
            Line::from_coords(0.0, 1.0, 1.0, 1.0),
            Line::from_coords(1.0, 1.0, 1.0, 0.0),
            Line::from_coords(1.0, 0.0, 1.0, 1.0),
        ]
    }

    #[test]
    fn simple_square(){
        let plan = create_square();
        
        let point = Point::new(0.5, 0.5);

        let result = find_isovist(&plan, &point);
        assert_eq!(result.unwrap().len(), 4);
    }

}