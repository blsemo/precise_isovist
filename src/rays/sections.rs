use std::convert::From;
use std::{collections::HashSet, fmt::Debug};

use ordered_float::OrderedFloat;

use crate::rays::error::GeometryError;

use super::types::*;

#[derive(Debug)]
pub struct Intersection<'l> {
    pub point: Point,
    pub scale_factor: f64,
    pub lines: HashSet<&'l Line>,
}

impl<'l> Intersection<'l> {
    pub fn merge(&self, other: &Intersection<'l>) -> Result<Intersection<'l>, GeometryError> {
        if self.point.distance(&other.point) > EPSILON {
            return Err(GeometryError::TooFarApartToMerge.into());
        }
        let mut lines = self.lines.clone();
        lines.extend(&other.lines);
        Ok(Intersection {
            point: self.point,
            scale_factor: self.scale_factor,
            lines,
        })
    }
}

const EPSILON: f64 = 0.00000001;

fn section_point<'l>(ray: &Line, line: &'l Line) -> Option<Intersection<'l>> {
    let d =
        (ray.b.x - ray.a.x) * (line.b.y - line.a.y) - (line.b.x - line.a.x) * (ray.b.y - ray.a.y);
    let r = ((line.b.x - line.a.x) * (ray.a.y - line.a.y)
        - (ray.a.x - line.a.x) * (line.b.y - line.a.y))
        / d;
    if (r + EPSILON) < OrderedFloat::from(0.0) {
        return None;
    }

    let s = ((line.a.x - ray.a.x) * (ray.b.y - ray.a.y)
        - (ray.b.x - ray.a.x) * (line.a.y - ray.a.y))
        / d;
    if (s + EPSILON).into_inner() < 0.0 || (s - EPSILON).into_inner() > 1.0 {
        return None;
    }

    let mut lines = HashSet::<&'l Line>::new();
    lines.insert(line);

    return Some(Intersection {
        point: Point::new_ordered(
            s * (line.b.x - line.a.x) + line.a.x,
            s * (line.b.y - line.a.y) + line.a.y,
        ),
        scale_factor: r.into_inner(),
        lines,
    });
}

pub fn closest_section<'l>(ray: &Line, lines: &'l Vec<Line>) -> Option<Intersection<'l>> {
    let mut intersection: Option<Intersection> = None;

    for line in lines {
        let i = section_point(ray, line);
        if let Some(iv) = i {
            if intersection.is_none()
                || iv.scale_factor < intersection.as_ref().unwrap().scale_factor
            {
                intersection = Some(iv);
            } else if intersection.as_ref().unwrap().scale_factor == iv.scale_factor {
                intersection.as_mut().unwrap().lines.extend(iv.lines);
            }
        }
    }

    return intersection;
}

fn get_ray_points(line: &Line) -> Vec<Point> {
    vec![
        Point::new_ordered(
            OrderedFloat(-2.0 * EPSILON) * (line.b.x - line.a.x) + line.a.x,
            OrderedFloat(-2.0 * EPSILON) * (line.b.y - line.a.y) + line.a.y,
        ),
        line.a,
        line.b,
        Point::new_ordered(
            OrderedFloat(1.0 + 2.0 * EPSILON) * (line.b.x - line.a.x) + line.a.x,
            OrderedFloat(1.0 + 2.0 * EPSILON) * (line.b.y - line.a.y) + line.a.y,
        ),
    ]
}

pub fn closest_intersections_to_all_vertices<'l>(
    point: &Point,
    lines: &'l Vec<Line>,
) -> Vec<Intersection<'l>> {
    let mut results = Vec::<Intersection>::new();
    let mut seen_points = HashSet::new();
    for line in lines {
        for p in get_ray_points(line) {
            if seen_points.insert(p) {
                if let Some(intersection) = closest_section(&Line::from_points(point, &p), lines) {
                    results.push(intersection);
                }
            }
        }
    }
    return results;
}

pub fn sort_intersections<'l>(
    point: &Point,
    intersections: &'l Vec<Intersection>,
) -> Vec<&'l Intersection<'l>> {
    let mut points = Vec::<&Intersection>::new();
    for intersection in intersections {
        points.push(intersection);
    }

    points.sort_by(|a, b| {
        (b.point.y - point.y)
            .atan2(*(b.point.x - point.x))
            .partial_cmp(&(a.point.y - point.y).atan2(*(a.point.x - point.x)))
            .expect("Comparison failed")
    });
    return points;
}

pub fn collate_intersections<'l>(
    sorted_intersections: &Vec<&Intersection<'l>>,
) -> Vec<Intersection<'l>> {
    vec![]
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn merge_intersections() {
        let line1 = Line::from_coords(0.0, 0.0, 0.0, 1.0);
        let mut lines1 = HashSet::<&Line>::new();
        lines1.insert(&line1);

        let line2 = Line::from_coords(0.0, 1.0, 1.0, 1.0);
        let mut lines2 = HashSet::<&Line>::new();
        lines2.insert(&line2);

        let intersection1 = Intersection {
            point: Point::new(0.0, 1.0),
            scale_factor: 1.0,
            lines: lines1,
        };

        let intersection2 = Intersection {
            point: Point::new(0.00000000001, 1.0),
            scale_factor: 1.1,
            lines: lines2,
        };

        let result = intersection1
            .merge(&intersection2)
            .expect("Failed to merge points");

        assert_eq!(result.point, intersection1.point);
        assert_eq!(result.scale_factor, intersection1.scale_factor);

        assert_eq!(result.lines.len(), 2);
        assert!(result.lines.contains(&line1));
        assert!(result.lines.contains(&line2));
    }

    #[test]
    fn merge_intersections_fail_too_far_away() {
        let line1 = Line::from_coords(0.0, 0.0, 0.0, 1.0);
        let mut lines1 = HashSet::<&Line>::new();
        lines1.insert(&line1);

        let line2 = Line::from_coords(0.0, 1.0, 1.0, 1.0);
        let mut lines2 = HashSet::<&Line>::new();
        lines2.insert(&line2);

        let intersection1 = Intersection {
            point: Point::new(0.0, 1.0),
            scale_factor: 1.0,
            lines: lines1,
        };

        let intersection2 = Intersection {
            point: Point::new(0.01, 1.0),
            scale_factor: 1.1,
            lines: lines2,
        };

        let result = intersection1.merge(&intersection2);

        assert_eq!(
            result.expect_err("Merge worked where it shouldn't"),
            GeometryError::TooFarApartToMerge
        );
    }

    #[test]
    fn test_section_point() {
        let l = Line::from_points(&Point::new(1.0, 1.0), &Point::new(0.0, 1.0));
        let r = Line::from_points(&Point::new(0.5, 0.0), &Point::new(0.5, 1.5));

        let intersection = section_point(&r, &l).expect("Calculation failed");
        assert!(intersection.point.x == 0.5);
        assert!(intersection.point.y == 1.0);
        assert!(
            intersection.scale_factor > 2.0 / 3.0 - EPSILON
                || intersection.scale_factor > 2.0 / 3.0 + EPSILON
        );
    }

    #[test]
    fn section_point_angled() {
        let l = Line::from_points(&Point::new(0.0, 0.5), &Point::new(0.5, 1.0));
        let r = Line::from_points(&Point::new(0.5, 0.5), &Point::new(0.0, 1.0));

        let intersection = section_point(&r, &l).expect("Calculation failed");
        assert!(intersection.point.x == 0.25);
        assert!(intersection.point.y == 0.75);
        assert!(
            intersection.scale_factor > 0.5 - EPSILON || intersection.scale_factor < 0.5 + EPSILON
        );
    }

    #[test]
    fn test_invalid_sections() {
        let l = Line::from_points(&Point::new(1.0, 1.0), &Point::new(0.0, 1.0));
        let r = Line::from_points(&Point::new(0.5, 0.5), &Point::new(0.5, 0.0));

        assert!(section_point(&r, &l).is_none());

        let l2 = Line::from_coords(0.0, -1.0, 0.3, -1.0);
        assert!(section_point(&r, &l2).is_none());
    }

    #[test]
    fn find_closest_section() {
        let lines = vec![
            Line::from_coords(0.0, 1.0, 1.0, 1.0),
            Line::from_coords(0.0, 1.5, 1.0, 2.0),
        ];

        let r1 = Line::from_coords(0.5, 0.0, 0.5, 0.5);

        let i = closest_section(&r1, &lines).expect("No point returned!");
        assert!(i.point.x == 0.5);
        assert!(i.point.y == 1.0);

        let r2 = Line::from_coords(0.5, 0.0, 10.0, 0.2);

        assert!(closest_section(&r2, &lines).is_none());
    }

    #[test]
    fn find_closest_section_corner() {
        let lines = vec![
            Line::from_coords(0.0, 0.0, 0.0, 1.0),
            Line::from_coords(0.0, 1.0, 1.0, 1.0),
        ];

        let r1 = Line::from_coords(1.0, 0.0, -1.0, 2.0);

        let i = closest_section(&r1, &lines).expect("No point returned!");
        assert!(i.point.x == 0.0);
        assert!(i.point.y == 1.0);
        assert_eq!(i.lines.len(), 2);
        assert!(i.lines.contains(&lines[0]));
        assert!(i.lines.contains(&lines[1]));

        let r2 = Line::from_coords(0.5, 0.0, 0.5, 2.0);

        let i2 = closest_section(&r2, &lines).expect("No point returned");
        assert!(i2.point.x == 0.5);
        assert!(i2.point.y == 1.0);
        assert_eq!(i2.lines.len(), 1);
        assert!(i2.lines.contains(&lines[1]));
    }

    #[test]
    fn find_closest_section_complex() {
        let lines = vec![
            Line::from_coords(0.0, 0.0, 0.0, 1.0),
            Line::from_coords(0.0, 1.0, 1.0, 1.0),
            Line::from_coords(0.1, 0.6, 0.4, 0.9),
        ];

        let i = closest_section(&Line::from_coords(0.5, 0.5, 0.0, 1.0), &lines)
            .expect("section failed");
        assert_eq!(i.point.x, 0.25);
        assert_eq!(i.point.y, 0.75);
    }

    #[test]
    fn find_closest_intersection_to_all_vertices() {
        // Simple case - in a square, that's just the corners
        let mut lines = vec![
            Line::from_coords(0.0, 0.0, 0.0, 1.0),
            Line::from_coords(0.0, 1.0, 1.0, 1.0),
            Line::from_coords(1.0, 1.0, 1.0, 0.0),
            Line::from_coords(1.0, 0.0, 0.0, 0.0),
        ];

        let sections = closest_intersections_to_all_vertices(&Point::new(0.5, 0.5), &lines);

        for section in &sections {
            println!("{:?}", section);
        }

        assert_eq!(sections.len(), 12);
        assert_eq!(sections[1].point.x, 0.0);
        assert_eq!(sections[1].point.y, 0.0);
        assert_eq!(sections[2].point.x, 0.0);
        assert_eq!(sections[2].point.y, 1.0);
        assert_eq!(sections[5].point.x, 1.0);
        assert_eq!(sections[5].point.y, 1.0);
        assert_eq!(sections[8].point.x, 1.0);
        assert_eq!(sections[8].point.y, 0.0);

        // add line covering one corner

        lines.push(Line::from_coords(0.1, 0.6, 0.4, 0.9));

        let sections2 = closest_intersections_to_all_vertices(&Point::new(0.5, 0.5), &lines);

        assert_eq!(sections2.len(), 16);
        assert_eq!(sections2[1].point.x, 0.0);
        assert_eq!(sections2[1].point.y, 0.0);
        assert_eq!(sections2[2].point.x, 0.25);
        assert_eq!(sections2[2].point.y, 0.75);
        assert_eq!(sections2[12].point.x, 0.0);
        assert_eq!(sections2[13].point.x, 0.1);
        assert_eq!(sections2[13].point.y, 0.6);
        assert_eq!(sections2[14].point.x, 0.4);
        assert_eq!(sections2[14].point.y, 0.9);
        assert_eq!(sections2[15].point.y, 1.0);
    }

    #[test]
    fn test_point_sorting() {
        let line1 = Line::from_coords(0.5, 1.0, 1.0, 1.0);
        let line2 = Line::from_coords(0.1, 0.1, 0.5, 1.0);

        let mut set = HashSet::<&Line>::new();
        set.insert(&line1);
        set.insert(&line2);

        let intersections = vec![
            Intersection {
                point: Point::new(1.0, 1.0),
                scale_factor: 0.0,
                lines: set.clone(),
            },
            Intersection {
                point: Point::new(0.0, 1.0),
                scale_factor: 0.0,
                lines: set.clone(),
            },
            Intersection {
                point: Point::new(0.5, 1.0),
                scale_factor: 0.0,
                lines: set.clone(),
            },
            Intersection {
                point: Point::new(0.1, 0.1),
                scale_factor: 0.0,
                lines: set.clone(),
            },
        ];

        let sorted = sort_intersections(&Point::new(0.5, 0.5), &intersections);

        println!("{:?}", sorted);

        assert_eq!(sorted.len(), 4);

        assert_eq!(sorted[0].point.x, 0.0);
        assert_eq!(sorted[0].point.y, 1.0);

        assert_eq!(sorted[1].point.x, 0.5);
        assert_eq!(sorted[1].point.y, 1.0);

        assert_eq!(sorted[2].point.x, 1.0);
        assert_eq!(sorted[2].point.y, 1.0);

        assert_eq!(sorted[3].point.x, 0.1);
        assert_eq!(sorted[3].point.y, 0.1);
    }
}
