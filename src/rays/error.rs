use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub enum GeometryError {
    TooFarApartToMerge,
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GeometryError::TooFarApartToMerge => {
                write!(f, "Points are too far apart to be merged!")
            }
        }
    }
}

impl Error for GeometryError {}
