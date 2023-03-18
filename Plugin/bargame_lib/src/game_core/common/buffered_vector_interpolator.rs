use std::collections::VecDeque;
use std::cmp::Ordering;

pub struct BufferedVectorInterpolator {
    key_frames: VecDeque<InterpolationKeyFrame>,
}

pub struct InterpolationKeyFrame {
    pub value: nalgebra::Vector3<f32>,
    pub time: f32,
}

impl BufferedVectorInterpolator {
    pub fn new() -> Self {
        Self {
            key_frames: VecDeque::with_capacity(16),
        }
    }

    pub fn try_interpolate(&self, target_time: f32) -> Option<nalgebra::Vector3<f32>> {
        let mut interpolated_value = nalgebra::Vector3::zeros();

        // assert that keyframes times are sorted
        if !self.key_frames.iter().zip(self.key_frames.iter().skip(1)).all(|(a, b)| a.time <= b.time) {
            eprintln!("Keyframes are not sorted");
            return None;
        }

        // iterate in pairs
        for i in 0..(self.key_frames.len() - 1) {
            let start = &self.key_frames[i];
            let end = &self.key_frames[i + 1];

            // if the target time is between the start and end time, interpolate
            if target_time >= start.time && target_time <= end.time {
                let time_diff = end.time - start.time;
                let time = target_time - start.time;
                let t = time / time_diff;

                interpolated_value = nalgebra::Vector3::new(
                    start.value.x + (end.value.x - start.value.x) * t,
                    start.value.y + (end.value.y - start.value.y) * t,
                    start.value.z + (end.value.z - start.value.z) * t,
                );

                return Some(interpolated_value);
            }
        }

        None
    }

    pub fn interpolate(&self, target_time: f32) -> nalgebra::Vector3<f32> {
        self.try_interpolate(target_time).unwrap_or_else(|| nalgebra::Vector3::new(1000.0, 1000.0, 1000.0))
    }

    pub fn push(&mut self, value: nalgebra::Vector3<f32>, time: f32) {
        self.key_frames.push_back(InterpolationKeyFrame { value, time });
    }

    pub fn clear_before(&mut self, time: f32) {
        if self.key_frames.iter().any(|frame| frame.time < 0.0) {
            eprintln!("Keyframe time is negative");
        }
        self.key_frames.retain(|frame| frame.time >= time);
    }
}



#[cfg(test)]
mod tests {
    use nalgebra::Vector3;
    use super::*;

    #[test]
    fn test_interpolation() {
        let mut buffer = BufferedVectorInterpolator::new();

        buffer.push(Vector3::new(0.0, 0.0, 0.0), 0.0);
        buffer.push(Vector3::new(1.0, 0.0, 0.0), 1.0);
        buffer.push(Vector3::new(1.0, 1.0, 0.0), 2.0);
        buffer.push(Vector3::new(0.0, 1.0, 0.0), 3.0);

        assert_eq!(buffer.interpolate(0.0), Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(buffer.interpolate(0.5), Vector3::new(0.5, 0.0, 0.0));
        assert_eq!(buffer.interpolate(1.0), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(buffer.interpolate(1.5), Vector3::new(1.0, 0.5, 0.0));
        assert_eq!(buffer.interpolate(2.0), Vector3::new(1.0, 1.0, 0.0));
        assert_eq!(buffer.interpolate(2.5), Vector3::new(0.5, 1.0, 0.0));
        assert_eq!(buffer.interpolate(3.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(buffer.interpolate(3.5), Vector3::new(1000.0, 1000.0, 1000.0));
    }

    #[test]
    fn test_clear_before() {
        let mut buffer = BufferedVectorInterpolator::new();

        buffer.push(Vector3::new(0.0, 0.0, 0.0), 0.0);
        buffer.push(Vector3::new(1.0, 0.0, 0.0), 1.0);
        buffer.push(Vector3::new(1.0, 1.0, 0.0), 2.0);
        buffer.push(Vector3::new(0.0, 1.0, 0.0), 3.0);

        buffer.clear_before(1.5);

        assert_eq!(buffer.interpolate(0.0), Vector3::new(1000.0, 1000.0, 1000.0));
        assert_eq!(buffer.interpolate(1.0), Vector3::new(1000.0, 1000.0, 1000.0));
        assert_eq!(buffer.interpolate(1.5), Vector3::new(1000.0, 1000.0, 1000.0));
        assert_eq!(buffer.interpolate(2.0), Vector3::new(1.0, 1.0, 0.0));
        assert_eq!(buffer.interpolate(3.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(buffer.interpolate(4.0), Vector3::new(1000.0, 1000.0, 1000.0));
    }

    #[test]
    fn debug(){
        let a = Vector3::new(0.0, 0.0, 0.0);
        println!("a: {:?}", a);
        let b = Vector3::new(1.0, 0.0, 0.0);
        println!("b: {:?}", b);
    }
}
