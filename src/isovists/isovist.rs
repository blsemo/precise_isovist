struct Border {
    pub line: Line,
    pub isBounded: bool
}

pub fn findIsovist( plan: &Vec<Line>, point: &Point) -> Vec<Border> {

}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn simple_square(){
        let plan = vec![
            Line::from_coords(0.0, 0.0, 0.0, 1.0),
            Line::from_coords(0.0, 1.0, 1.0, 1.0),
            Line::from_coords(1.0, 1.0, 1.0, 0.0),
            Line::from_coords(1.0, 0.0, 1.0, 1.0),
        ];
        
        let point = Point::new(0.5, 0.5);

        let result = findIsovist(plan, point);
        assert_eq(result.len(), 4);
    }

}