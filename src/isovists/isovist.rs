use crate::rays::{
    sections::{
        Intersection, closest_intersections_to_all_vertices, collate_intersections, maximise_lines,
        sort_intersections,
    },
    types::{Line, LineConstructor, Point},
};
use std::{error::Error, fmt::Display};

pub struct Border {
    pub line: Line,
    pub is_bounded: bool,
}

impl Border {
    pub fn from_intersections(i1: &Intersection, i2: &Intersection) -> Border {
        let bounded = !i1
            .lines
            .intersection(&i2.lines)
            .collect::<Vec<_>>()
            .is_empty();
        Border {
            line: Line::from_points(&i1.point, &i2.point),
            is_bounded: bounded,
        }
    }
}

impl Display for Border {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Border ({}) from {:?} to {:?}",
            if self.is_bounded {
                "bounded"
            } else {
                "unbounded"
            },
            self.line.a,
            self.line.b
        ))
    }
}

#[derive(Debug)]
pub struct IsovistError {
    pub msg: String,
}

impl IsovistError {
    fn new(msg: &str) -> IsovistError {
        IsovistError {
            msg: msg.to_string(),
        }
    }
}

impl Display for IsovistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl Error for IsovistError {}

pub fn find_isovist(plan: &Vec<Line>, point: &Point) -> Result<Vec<Border>, IsovistError> {
    let intersections = closest_intersections_to_all_vertices(point, plan);
    let sorted_intersections = sort_intersections(point, &intersections);
    let collated_intersections = collate_intersections(&sorted_intersections);
    let maximised = maximise_lines(&collated_intersections);
    let mut previous_point: Option<&Intersection> = None;
    let mut result = Vec::<Border>::new();
    if maximised.len() < 3 {
        return Err(IsovistError::new("Too few points to construct isovist"));
    }

    for current in &maximised {
        if let Some(previous) = previous_point {
            result.push(Border::from_intersections(previous, current));
        }
        previous_point = Some(current);
    }
    result.push(Border::from_intersections(
        maximised.last().unwrap(),
        maximised.first().unwrap(),
    ));

    Ok(result)
}

#[cfg(test)]

mod tests {
    use super::*;

    fn create_square() -> Vec<Line> {
        vec![
            Line::from_coords(0.0, 0.0, 0.0, 1.0),
            Line::from_coords(0.0, 1.0, 1.0, 1.0),
            Line::from_coords(1.0, 1.0, 1.0, 0.0),
            Line::from_coords(1.0, 0.0, 0.0, 0.0),
        ]
    }

    #[test]
    fn simple_square() {
        let plan = create_square();

        let point = Point::new(0.5, 0.5);

        let result = find_isovist(&plan, &point);
        let unwrapped = result.unwrap();
        for b in &unwrapped {
            print!("{}\n", b);
        }
        assert_eq!(unwrapped.len(), 4);
    }

    #[test]
    fn square_with_floating_line() {
        let mut plan = create_square();

        // Add line shading part of the left half of the square
        plan.push(Line::from_coords(0.3, 0.4, 0.3, 0.8));

        let point = Point::new(0.5, 0.5);

        let result = find_isovist(&plan, &point);
        let unwrapped = result.unwrap();
        for b in &unwrapped {
            print!("{}\n", b);
        }
        assert_eq!(unwrapped.len(), 7);
        assert_eq!(
            unwrapped
                .iter()
                .map(|b| b.is_bounded)
                .collect::<Vec<bool>>(),
            vec!(false, true, true, true, true, false, true)
        );
    }
}
