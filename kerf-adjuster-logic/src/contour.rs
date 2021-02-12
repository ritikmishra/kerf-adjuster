use crate::errors::{KerfAdjustmentError, KerfAdjustmentErrorReason};
use dxf::entities::*;
use dxf::Drawing;
use dxf::Point;
use dxf::Vector;
use nalgebra::Vector3;

/// This is a newtype that exists solely to convert nalgebra vectors to/from DXF crate counterparts
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

pub fn find_endpoints_of_entity(e: &Entity) -> Option<(Vector3<f64>, Vector3<f64>)> {
    // TODO: real error handling for unsupported entities - Result instead of Option

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

enum ContourSides {
    Inside,
    Outside,
}

#[derive(Clone)]
pub struct Contour {
    pub entities: Vec<Entity>,
    // Can be none if the contour is closed
    pub end_points: Option<(Vector3<f64>, Vector3<f64>)>,
}

impl std::fmt::Debug for Contour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Contour")
            .field("entities", &self.entities.len())
            .field("end_points", &self.end_points)
            .finish()
    }
}

impl From<Entity> for Contour {
    fn from(e: Entity) -> Self {
        Contour::create_from_entity(e)
    }
}

impl Contour {
    pub fn create_from_entity(e: Entity) -> Self {
        return Self {
            end_points: find_endpoints_of_entity(&e),
            entities: vec![e],
        };
    }

    pub fn is_open(&self) -> bool {
        return self.end_points.is_some();
    }

    // TODO: epsilon should be configurable
    const EPSILON: f64 = 1e-6;

    pub fn combine_attempt(self, other: Self) -> Result<Self, (Self, Self)> {
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
                    new_entities.extend(self.entities);
                    new_entities.extend(other.entities.into_iter().rev());

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

    /// Negative amount will shrink the area of the contour. Positive amount will grow the area of the contour.
    pub fn offset_contour(&self, amount: f64) -> Result<Self, KerfAdjustmentError> {
        if self.is_open() {
            return Err(KerfAdjustmentError {
                reason: KerfAdjustmentErrorReason::CannotOffsetOpenContour,
            });
        }

        let mut entities_iterator = self.entities.clone().into_iter();

        let first_entity = entities_iterator.next().ok_or(KerfAdjustmentError {
            reason: KerfAdjustmentErrorReason::CannotOffsetEmptyContour,
        })?;

        // Create a contour and put the first entity (offset, of) into it
        let mut result_contour: Contour = naive_entity_offset(first_entity.clone(), amount)
            .ok_or(KerfAdjustmentError {
                reason: KerfAdjustmentErrorReason::CannotOffsetEntity(first_entity.specific),
            })?
            .0 // TODO: handle both the inside and outside contours
            .into();

        for entity in entities_iterator {
            // Offset the entity, get 2 offset versions
            let (offset_a, offset_b) =
                naive_entity_offset(entity.clone(), amount).ok_or(KerfAdjustmentError {
                    reason: KerfAdjustmentErrorReason::CannotOffsetEntity(entity.specific),
                })?;

            // Try to connect one or the other to the
            let maybe_combined_offset_contour_into_result = result_contour
                .combine_attempt(offset_a.into())
                .or_else(|(result_contour, _)| result_contour.combine_attempt(offset_b.into()));

            match maybe_combined_offset_contour_into_result {
                Ok(success) => result_contour = success,
                Err(_) => {
                    return Err(KerfAdjustmentError {
                        reason: KerfAdjustmentErrorReason::CannotConnectContourAfterAdjustment,
                    })
                }
            }
        }

        return Ok(result_contour);
    }
}

/// Returns either
///     - A pair of entites that are offset_amount away from the provided entity
///     - None, if the entity cannot be offset
fn naive_entity_offset(entity: Entity, amount: f64) -> Option<(Entity, Entity)> {
    let offset_entity_specifics: (EntityType, EntityType) = match entity.specific {
        EntityType::Circle(circle) => (
            EntityType::Circle(Circle {
                radius: circle.radius + amount,
                ..circle.clone()
            }),
            EntityType::Circle(Circle {
                radius: circle.radius - amount,
                ..circle
            }),
        ),
        EntityType::Arc(arc) => (
            EntityType::Arc(Arc {
                radius: arc.radius + amount,
                ..arc.clone()
            }),
            EntityType::Arc(Arc {
                radius: arc.radius - amount,
                ..arc
            }),
        ),
        EntityType::Line(line) => {
            let VectorWrapper(start_vec) = line.p1.clone().into();
            let VectorWrapper(end_vec) = line.p2.clone().into();
            let VectorWrapper(plane_normal) = line.extrusion_direction.clone().into();
            let delta = end_vec - start_vec;
            // The plane normal should be [0, 0, 1] & delta should be [x, y, 0] --> the perp delta should be [-y, x, 0]
            let mut perpendicular_delta = delta.cross(&plane_normal);
            perpendicular_delta.set_magnitude(amount);

            let (new_start_1, new_end_1) = (
                VectorWrapper(start_vec + perpendicular_delta),
                VectorWrapper(end_vec + perpendicular_delta),
            );

            let (new_start_2, new_end_2) = (
                VectorWrapper(start_vec - perpendicular_delta),
                VectorWrapper(end_vec - perpendicular_delta),
            );

            (
                EntityType::Line(Line {
                    p1: new_start_1.into(),
                    p2: new_end_1.into(),
                    ..line.clone()
                }),
                EntityType::Line(Line {
                    p1: new_start_2.into(),
                    p2: new_end_2.into(),
                    ..line
                }),
            )
        }
        _ => return None,
    };

    Some((
        Entity {
            common: entity.common.clone(),
            specific: offset_entity_specifics.0,
        },
        Entity {
            common: entity.common,
            specific: offset_entity_specifics.1,
        },
    ))
}

pub trait ContourVecToDxf {
    fn to_dxf(self) -> Drawing;
}

impl ContourVecToDxf for Vec<Contour> {
    fn to_dxf(self) -> dxf::Drawing {
        let mut drawing: Drawing = Default::default();

        for contour in self {
            drawing.entities.extend(contour.entities);
        }
        drawing
    }
}

#[cfg(test)]
mod contour_test {

    use crate::find_endpoints_of_entity;
    use crate::Contour;
    use dxf::entities::*;
    use dxf::Point;
    use dxf::Vector;

    const EPSILON: f64 = 1e-6;

    fn line_between(p1: Point, p2: Point) -> Entity {
        Entity {
            common: Default::default(),
            specific: EntityType::Line(Line {
                thickness: 0., // hairline
                p1,
                p2,
                extrusion_direction: Vector::new(0., 0., 1.),
            }),
        }
    }

    fn check_contour_is_sequential(c: &Contour) -> Option<()> {
        let endpoints = c.entities.iter().map(find_endpoints_of_entity).try_fold(
            Vec::new(),
            |mut acc, val| {
                acc.push(val?);
                Some(acc)
            },
        )?;

        let mut endpoints_iter = endpoints.into_iter();
        let mut prev_end = endpoints_iter.next()?.1;

        for (next_start, next_end) in endpoints_iter {
            if (prev_end - next_start).magnitude() > EPSILON {
                return None;
            }
            prev_end = next_end;
        }

        Some(())
    }

    #[test]
    pub fn test_contour_sequential_check_works() {
        // given: two entities that connect the way we want them to
        let line1 = line_between(Point::origin(), Point::new(1., 2., 3.));

        let line2 = line_between(Point::new(1., 2., 3.), Point::new(4., 5., 6.));

        // when: we connect them in a contour
        let contour_12 = Contour::from(line1.clone())
            .combine_attempt(line2.clone().into())
            .unwrap();
        let contour_21 = Contour::from(line2.clone())
            .combine_attempt(line1.clone().into())
            .unwrap();

        // then: it passes the test
        check_contour_is_sequential(&contour_12).unwrap();
        check_contour_is_sequential(&contour_21).unwrap();
    }

    #[test]
    pub fn test_ac_combine() {
        // given: two entities that connect at their beginning
        let line1 = line_between(Point::origin(), Point::new(1., 3., 0.));
        let line2 = line_between(Point::origin(), Point::new(-3., 1., 0.));

        // when: we combine them into a contour
        let combined_entity = Contour::from(line1).combine_attempt(line2.into()).unwrap();

        // then: sequential entities
        check_contour_is_sequential(&combined_entity).unwrap();
    }

    #[test]
    pub fn test_ad_combine() {
        // given: entity 2 ends where entity 1 starts
        let line1 = line_between(Point::origin(), Point::new(1., 3., 0.));
        let line2 = line_between(Point::new(-3., 1., 0.), Point::origin());

        // when: we combine them into a contour
        let combined_entity = Contour::from(line1).combine_attempt(line2.into()).unwrap();

        // then: sequential entities
        check_contour_is_sequential(&combined_entity).unwrap();
    }

    #[test]
    pub fn test_bc_combine() {
        // given: entity 1 ends where entity 2 starts
        let line1 = line_between(Point::new(1., 3., 0.), Point::origin());
        let line2 = line_between(Point::origin(), Point::new(-3., 1., 0.));

        // when: we combine them into a contour
        let combined_entity = Contour::from(line1).combine_attempt(line2.into()).unwrap();

        // then: sequential entities
        check_contour_is_sequential(&combined_entity).unwrap();
    }

    #[test]
    pub fn test_bd_combine() {
        // given: two entities that connect at their beginning
        let line1 = line_between(Point::origin(), Point::new(1., 3., 0.));
        let line2 = line_between(Point::origin(), Point::new(-3., 1., 0.));

        // when: we combine them into a contour
        let combined_entity = Contour::from(line1).combine_attempt(line2.into()).unwrap();

        // then: sequential entities
        check_contour_is_sequential(&combined_entity).unwrap();
    }
}
