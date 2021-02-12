use dxf::entities::EntityType;

fn entity_type_name(e: &EntityType) -> String {
    format!("{:?}", e)
        .split("(")
        .next()
        .unwrap_or("unknown entity type")
        .to_string()
}

#[derive(Debug)]
pub enum KerfAdjustmentErrorReason {
    UnsupportedEntity(EntityType),
    ThreeDimensionalEntity,
    CannotOffsetOpenContour,
    CannotOffsetEmptyContour,
    CannotConnectContourAfterAdjustment,
    CannotOffsetEntity(EntityType),
}

impl std::fmt::Display for KerfAdjustmentErrorReason {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::UnsupportedEntity(entity_type) => {
                write!(
                    fmt,
                    "Unsupported Entity (entity type: {})",
                    entity_type_name(entity_type)
                )
            }
            Self::ThreeDimensionalEntity => {
                write!(fmt, "3D entity found. Only 2D DXF files are supported")
            }
            Self::CannotOffsetOpenContour => write!(
                fmt,
                "Attempting to offset an open contour. Only closed contours can be offset."
            ),
            Self::CannotOffsetEmptyContour => write!(
                fmt,
                "Attempting to an empty contour. A contour must have at least one entity in it in order to be offset."
            ),
            Self::CannotConnectContourAfterAdjustment => write!(
                fmt,
                "Successfully adjusted contour, but could not connect it to the rest of the contour"
            ),
            Self::CannotOffsetEntity(entity_type) => {
                write!(
                    fmt,
                    "Cannot offset entity (entity type: {})",
                    entity_type_name(entity_type)
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct KerfAdjustmentError {
    pub reason: KerfAdjustmentErrorReason,
}

impl std::fmt::Display for KerfAdjustmentError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "Kerf Adjustment Error: {}", self.reason)
    }
}

impl std::error::Error for KerfAdjustmentError {}
