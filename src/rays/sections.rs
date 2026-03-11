use std::{collections::HashSet, fmt::Debug};

use ordered_float::OrderedFloat;

use super::types::*;

#[derive(Debug)]
pub struct Intersection {
    pub point: Point,
    pub scale_factor: f64,
}

const EPSILON: f64 = 0.00000001;

fn section_point(ray: &Line, line: &Line) -> Option<Intersection> {
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

    return Some(Intersection {
        point: Point {
            x: s * (line.b.x - line.a.x) + line.a.x,
            y: s * (line.b.y - line.a.y) + line.a.y,
        },
        scale_factor: r.into_inner(),
    });
}

pub fn closest_section(ray: &Line, lines: &Vec<Line>) -> Option<Intersection> {
    let mut intersection: Option<Intersection> = None;

    for line in lines {
        let i = section_point(ray, line);
        if let Some(iv) = i {
            if intersection.is_none()
                || iv.scale_factor < intersection.as_ref().unwrap().scale_factor
            {
                intersection = Some(iv);
            }
        }
    }

    return intersection;
}

fn get_ray_points(line: &Line) -> Vec<Point> {
    vec!(
        Point{
            x: OrderedFloat(-2.0 * EPSILON) * (line.b.x - line.a.x) + line.a.x,
            y: OrderedFloat(-2.0 * EPSILON) * (line.b.y - line.a.y) + line.a.y,
                },
        line.a,
        line.b,
        Point{
            x: OrderedFloat(1.0 + 2.0 * EPSILON) * (line.b.x - line.a.x) + line.a.x,
            y: OrderedFloat(1.0 + 2.0 * EPSILON) * (line.b.y - line.a.y) + line.a.y,
        },
    )
}

pub fn closest_intersections_to_all_vertices(
    point: &Point,
    lines: &Vec<Line>,
) -> Vec<Intersection> {
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

pub fn sort_intersections(
    point: &Point,
    intersections: &Vec<Intersection>,
) -> Vec<Point> {
    let mut points = Vec::<Point>::new();
    for intersection in intersections {
        points.push(intersection.point.clone());
    }

    points.sort_by(|a, b| (b.y - point.y).atan2(*(b.x - point.x)).partial_cmp(&(a.y - point.y).atan2(*(a.x - point.x))).expect("Comparison failed")  );
    return points;
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_section_point() {
        let l = Line {
            a: Point::new(1.0, 1.0),
            b: Point::new(0.0, 1.0),
        };
        let r = Line {
            a: Point::new(0.5, 0.0),
            b: Point::new(0.5, 1.5),
        };

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
        let l = Line {
            a: Point::new(0.0, 0.5),
            b: Point::new(0.5, 1.0),
        };
        let r = Line {
            a: Point::new(0.5, 0.5),
            b: Point::new(0.0, 1.0),
        };

        let intersection = section_point(&r, &l).expect("Calculation failed");
        assert!(intersection.point.x == 0.25);
        assert!(intersection.point.y == 0.75);
        assert!(
            intersection.scale_factor > 0.5 - EPSILON || intersection.scale_factor < 0.5 + EPSILON
        );
    }

    #[test]
    fn test_invalid_sections() {
        let l = Line {
            a: Point::new(1.0, 1.0),
            b: Point::new(0.0, 1.0),
        };
        let r = Line {
            a: Point::new(0.5, 0.5),
            b: Point::new(0.5, 0.0),
        };

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
        let intersections = vec!(
            Intersection{ point: Point::new(1.0, 1.0), scale_factor: 0.0 },
            Intersection{ point: Point::new(0.0, 1.0), scale_factor: 0.0 },
            Intersection{ point: Point::new(0.5, 1.0), scale_factor: 0.0 },
            Intersection{ point: Point::new(0.1, 0.1), scale_factor: 0.0 },
        );

        let sorted = sort_intersections(&Point::new(0.5, 0.5), &intersections);


        println!("{:?}", sorted);

        assert_eq!(sorted.len(), 4);

        assert_eq!(sorted[0].x, 0.0);
        assert_eq!(sorted[0].y, 1.0);

        assert_eq!(sorted[1].x, 0.5);
        assert_eq!(sorted[1].y, 1.0);

        assert_eq!(sorted[2].x, 1.0);
        assert_eq!(sorted[2].y, 1.0);

        assert_eq!(sorted[3].x, 0.1);
        assert_eq!(sorted[3].y, 0.1);


    }
}
