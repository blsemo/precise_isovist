use dxf::{Drawing, Point, entities::*, tables::Layer};


fn main() {
    // let path = "test_data/gallery.dxf";

    // let drawing = Drawing::load_file(path).expect("Failed to load drawing");
    // for e in drawing.entities() {
    //     println!("found entity on layer {}", e.common.layer);
    //     match e.specific {
    //         EntityType::Circle(ref circle) => {
    //             println!("Circle, center: {},{}, radius: {}", circle.center.x, circle.center.y, circle.radius);
    //         }
    //         EntityType::Line(ref line) => {
    //            println!("Line from {},{} to {},{}", line.p1.x, line.p2.y, line.p2.x, line.p2.y);
    //         }
    //         _ => (),
    //     }
    // }

    let drawing = create_test_dxf();
    drawing.save_file("test.dxf");
}

fn create_test_dxf() -> Drawing {
    let mut drawing = Drawing::new();

    // draw a square box ((0,0)(0,1)(1,1)(1,0))
    drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(Line::new(Point::new(0.0, 0.0, 0.0), Point::new(0.0,1.0, 0.0)))));
    drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(Line::new(Point::new(0.0, 1.0, 0.0), Point::new(1.0,1.0, 0.0)))));
    drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(Line::new(Point::new(1.0, 1.0, 0.0), Point::new(1.0,0.0, 0.0)))));
    drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(Line::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0,0.0, 0.0)))));

    // draw a small open shape for interesting isovists
    drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(Line::new(Point::new(0.2, 0.2, 0.0), Point::new(0.2,0.6, 0.0)))));
    drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(Line::new(Point::new(0.2, 0.6, 0.0), Point::new(0.6,0.6, 0.0)))));
    drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(Line::new(Point::new(0.201, 0.2, 0.0), Point::new(0.6,0.2, 0.0)))));
    drawing.add_entity(Entity::new(dxf::entities::EntityType::Line(Line::new(Point::new(0.6, 0.55, 0.0), Point::new(0.6,0.25, 0.0)))));

    return drawing
}