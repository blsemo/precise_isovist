use std::{error::Error, fmt::Display};

use clap::Parser;
use dxf::{Drawing, LwPolylineVertex, Point, entities::*};

use crate::rays::{
    conversions::DxfConverter,
    types::{Line, LineConstructor},
};

mod isovists;
mod rays;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short('f'), long)]
    input_file: String,

    #[arg(short, long)]
    isovist_layer: String,

    #[arg(short, long)]
    plan_layer: String,

    #[arg(short, long)]
    output_file: String,
}

#[derive(Debug)]
pub struct ProgError {
    pub msg: String,
}

impl ProgError {
    pub fn new(msg: &str) -> ProgError {
        ProgError {
            msg: msg.to_string(),
        }
    }
}

impl Error for ProgError {}

impl Display for ProgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ProgramError: {}", self.msg))
    }
}

fn main() -> Result<(), ProgError> {
    let args = Args::parse();

    let drawing = Drawing::load_file(&args.input_file)
        .map_err(|e| ProgError::new(&format!("Failed to load file {}, {}", args.input_file, e)))?;

    let mut lines = Vec::<Line>::new();

    for e in drawing.entities() {
        println!("found entity on layer {}", e.common.layer);
        if e.common.layer == args.plan_layer {
            println!("Plan layer {}, taking entity", args.plan_layer);
            match &e.specific {
                EntityType::Line(line) => {
                    lines.push(Line::from_dxf(e).map_err(|_| {
                        ProgError::new(&format!("Failed to convert line {:?}", line))
                    })?);
                }
                EntityType::LwPolyline(polyline) => {
                    let mut previous: Option<&LwPolylineVertex> = None;
                    for v in &polyline.vertices {
                        if let Some(previous_vertex) = previous {
                            lines.push(Line::from_coords(
                                previous_vertex.x,
                                previous_vertex.y,
                                v.x,
                                v.y,
                            ));
                        }
                        previous = Some(v);
                    }
                }
                et => return Err(ProgError::new(&format!("Unsupported entity type {:?}", et))),
            }
        }
    }

    println!("Created {} lines", lines.len());

    Ok(())
}

// fn create_test_dxf() -> Drawing {
//     let mut drawing = Drawing::new();

//     // draw a square box ((0,0)(0,1)(1,1)(1,0))
//     drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(dxf::entities::Line::new(
//         Point::new(0.0, 0.0, 0.0),
//         Point::new(0.0, 1.0, 0.0),
//     ))));
//     drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(dxf::entities::Line::new(
//         Point::new(0.0, 1.0, 0.0),
//         Point::new(1.0, 1.0, 0.0),
//     ))));
//     drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(dxf::entities::Line::new(
//         Point::new(1.0, 1.0, 0.0),
//         Point::new(1.0, 0.0, 0.0),
//     ))));
//     drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(dxf::entities::Line::new(
//         Point::new(0.0, 0.0, 0.0),
//         Point::new(1.0, 0.0, 0.0),
//     ))));

//     // draw a small open shape for interesting isovists
//     drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(dxf::entities::Line::new(
//         Point::new(0.2, 0.2, 0.0),
//         Point::new(0.2, 0.6, 0.0),
//     ))));
//     drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(dxf::entities::Line::new(
//         Point::new(0.2, 0.6, 0.0),
//         Point::new(0.6, 0.6, 0.0),
//     ))));
//     drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(dxf::entities::Line::new(
//         Point::new(0.201, 0.2, 0.0),
//         Point::new(0.6, 0.2, 0.0),
//     ))));
//     drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(dxf::entities::Line::new(
//         Point::new(0.6, 0.55, 0.0),
//         Point::new(0.6, 0.25, 0.0),
//     ))));

//     return drawing;
// }
