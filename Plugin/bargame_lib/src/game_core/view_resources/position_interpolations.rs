use std::collections::HashMap;
use crate::game_core::common::buffered_vector_interpolator::BufferedVectorInterpolator;
use crate::game_core::view_resources::interpolated_position_id::InterpolatedPositionId;

pub struct PositionInterpolations {
    pub interpolations: HashMap<InterpolatedPositionId, BufferedVectorInterpolator>,
}