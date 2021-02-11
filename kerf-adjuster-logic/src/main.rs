use dxf;
use dxf::entities::*;
use dxf::Drawing;
use dxf::{Point, Vector};
use nalgebra::Vector3;
use std::collections::HashMap;
use std::default::Default;

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
            let (VectorWrapper(start), VectorWrapper(end)) =
                (line.p1.clone().into(), line.p2.clone().into());
            Some((start, end))
        }
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

            Some((start_pt, end_pt))
        }
        EntityType::Text(_) | EntityType::MText(_) => None,
        _ => todo!(),
    }
}

#[derive(Clone)]
struct Contour {
    entities: Vec<Entity>,
    // Can be none if the contour is closed
    end_points: Option<(Vector3<f64>, Vector3<f64>)>,
}

impl std::fmt::Debug for Contour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Contour")
            .field("entities", &self.entities.len())
            .field("end_points", &self.end_points)
            .finish()
    }
}

impl Contour {
    fn create_from_entity(e: Entity) -> Self {
        return Self {
            end_points: find_endpoints_of_entity(&e),
            entities: vec![e],
        };
    }
    // TODO: epsilon should be configurable
    const EPSILON: f64 = 1e-6;

    fn combine_attempt(self, other: Self) -> Result<Self, (Self, Self)> {
        match (self.end_points, other.end_points) {
            (Some((a, b)), Some((c, d))) => {
                let ac_dist = (a - c).magnitude();
                let ad_dist = (a - d).magnitude();
                let bc_dist = (b - c).magnitude();
                let bd_dist = (b - d).magnitude();

                let mut new_entities =
                    Vec::with_capacity(self.entities.len() + other.entities.len());
                if ac_dist < Self::EPSILON {
                    // our beginnning attaches to their beginning
                    // reverse ourself, so our beginning becomes our end
                    new_entities.extend(self.entities.into_iter().rev());
                    new_entities.extend(other.entities);

                    Ok(Self {
                        entities: new_entities,
                        end_points: if bd_dist >= Self::EPSILON {
                            Some((b, d))
                        } else {
                            None
                        },
                    })
                } else if ad_dist < Self::EPSILON {
                    // their beginning - their end - our beginning - our end
                    new_entities.extend(other.entities);
                    new_entities.extend(self.entities);

                    Ok(Self {
                        entities: new_entities,
                        end_points: if bc_dist >= Self::EPSILON {
                            Some((c, b))
                        } else {
                            None
                        }, // use their beginning, our end
                    })
                } else if bc_dist < Self::EPSILON {
                    // our end attaches to their beginning
                    new_entities.extend(self.entities);
                    new_entities.extend(other.entities);
                    Ok(Self {
                        entities: new_entities,
                        end_points: if ad_dist >= Self::EPSILON {
                            Some((a, d))
                        } else {
                            None
                        },
                    })
                } else if bd_dist < Self::EPSILON {
                    // our end attaches to their end
                    // reverse them, so their beginning attaches to our end
                    new_entities.extend(other.entities.into_iter().rev());
                    new_entities.extend(self.entities);

                    Ok(Self {
                        entities: new_entities,
                        end_points: if ac_dist >= Self::EPSILON {
                            Some((a, c))
                        } else {
                            None
                        },
                    })
                } else {
                    Err((self, other))
                }
            }
            _ => Err((self, other)), // if one of the contours is closed, cannot combine with it
        }
    }
}


trait ContourVecToDxf {
    fn to_dxf(self) -> Drawing;
}

impl ContourVecToDxf for Vec<Contour> {
    fn to_dxf(self) -> dxf::Drawing {
        let mut drawing: Drawing = Default::default();

        todo!();
    }
}

fn main() {
    // Load DXF
    let drawing =
        Drawing::load_file("/home/ritik/codeday_workspace/kerf-adjust/example_dxf.DXF").unwrap();

    println!(
        "There are {} entities in the drawing",
        drawing.entities.len()
    );

    // Convert each DXF entity (arc, circle, text, etc) into a "Contour" which can be more easily manipulated by us
    let contours = drawing
        .entities
        .clone()
        .into_iter()
        .map(Contour::create_from_entity)
        .collect::<Vec<_>>();

    // Partition the contours by whether or not they are open (i.e can be joined to another contour)
    let (mut contours, mut finished_contours) = contours
        .into_iter()
        .enumerate()
        .partition::<HashMap<usize, _>, _>(|(_, c)| c.end_points.is_some());

    // Group the contours with each other
    // TODO: handle unclosed contours
    while contours.len() > 0 {
        'each_a: for (i, current_contour) in contours.iter() {
            loop {
                for (j, other_contour) in contours.iter() {
                    let (i, j) = (*i, *j);
                    if i != j {
                        if let Ok(combined) = current_contour.clone().combine_attempt(other_contour.clone()) {
                            // We will combine other_contour into this_contour
                            // Remove other_contour from contours map
                            contours.remove(&j);

                            // Check if the combined result is open or closed, and 
                            // put it in its place accordingly
                            if combined.end_points.is_some() {
                                contours.insert(i, combined);
                            } else {
                                contours.remove(&i);
                                finished_contours.insert(i, combined);
                            }

                            // Restart the contour merging process from the beginning
                            // helps with ensuring consistent behavior
                            break 'each_a;
                        }
                    }
                }
            }
        }
    }

    println!("there are {} closed contours", finished_contours.len())
}
