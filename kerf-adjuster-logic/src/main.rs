use dxf;
use dxf::entities::*;
use dxf::Drawing;
use dxf::{Point, Vector};
use nalgebra::Vector3;
use std::rc::Rc;

struct Contour {
    entities: Vec<Entity>,
    // Can be none if the contour is closed
    end_points: Option<(Vector3<f64>, Vector3<f64>)>,
}

impl Contour {
    fn create_from_entity(e: Entity) -> Self {
        return Self {
            end_points: find_endpoints_of_entity(&e),
            entities: vec![e],
        }
    }
}

struct VectorWrapper(Vector3<f64>);

impl From<Vector> for VectorWrapper {
    fn from(vec: Vector) -> Self {
        VectorWrapper(Vector3::new(vec.x, vec.y, vec.z))
    }
}

impl From<Point> for VectorWrapper {
    fn from(pt: Point) -> Self {
        VectorWrapper(Vector3::new(pt.x, pt.y, pt.z))
    }
}

impl From<VectorWrapper> for Point {
    fn from(VectorWrapper(v): VectorWrapper) -> Self {
        Point {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<VectorWrapper> for Vector {
    fn from(VectorWrapper(nalg_vec): VectorWrapper) -> Self {
        Vector {
            x: nalg_vec.x,
            y: nalg_vec.y,
            z: nalg_vec.z,
        }
    }
}

fn find_endpoints_of_entity(e: &Entity) -> Option<(Vector3<f64>, Vector3<f64>)> {
    // TODO: real error handling for unsupported entities

    match &e.specific {
        EntityType::Circle(_) => None, // Circles are their own contour
        EntityType::Line(line) => {
            // convert line endpoints into vectors
            let (VectorWrapper(start), VectorWrapper(end)) = (line.p1.clone().into(), line.p2.clone().into());
            Some((start, end))
        },
        EntityType::Arc(arc) => {
            let VectorWrapper(normal_vec) = arc.normal.clone().into();

            // TODO: figure out the 3d math to make work 
            if normal_vec != Vector3::new(0., 0., 1.) {
                return None;
            }

            let VectorWrapper(center) = arc.center.clone().into();

            // start and end angles are counter-clockwise from x axis, like radians
            let start_rot = nalgebra::Unit::from_axis_angle(
                &nalgebra::Unit::new_normalize(normal_vec),
                arc.start_angle.to_radians(),
            );

            let end_rot = nalgebra::Unit::from_axis_angle(
                &nalgebra::Unit::new_normalize(normal_vec),
                arc.end_angle.to_radians(),
            );

            // point on the circle where angle = 0
            let mut circle_axis = Vector3::new(1., 0., 0.);
            circle_axis.set_magnitude(arc.radius);

            let (start_pt, end_pt) = (
                center + start_rot.transform_vector(&circle_axis),
                center + end_rot.transform_vector(&circle_axis),
            );

            Some((
                start_pt,
                end_pt
            ))
        }
        EntityType::Text(_) | EntityType::MText(_) => None,
        _ => todo!(),
    }
}

fn main() {
    println!("Hello, world!");

    let drawing =
        Drawing::load_file("/home/ritik/codeday_workspace/kerf-adjust/example_dxf.DXF").unwrap();

    println!(
        "There are {} entities in the drawing",
        drawing.entities().count()
    );
    let mut apple: Vec<Rc<Entity>> = Vec::new();
    for e in drawing.entities() {
        println!("endpoints: {:?}", find_endpoints_of_entity(e));
        apple.push(Rc::new(e.clone()));
    }
}
