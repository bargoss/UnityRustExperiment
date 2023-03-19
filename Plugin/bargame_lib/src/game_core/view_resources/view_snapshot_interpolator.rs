use std::collections::VecDeque;
use crate::game_core::view_resources::view_snapshot::{interpolate_snapshots, ViewSnapshot};

pub struct InterpolationKeyFrame<T> where T: ViewSnapshot{
    pub value: T,
    pub time: f32,
}

pub struct BufferedViewSnapshotInterpolator<T> where T: ViewSnapshot {
    key_frames: VecDeque<InterpolationKeyFrame<T>>,
}

const MAX_KEYFRAMES: usize = 8;

// BufferedViewSnapshotInterpolator
impl <T> BufferedViewSnapshotInterpolator<T> where T: ViewSnapshot {
    pub fn new() -> Self {
        Self {
            key_frames: VecDeque::with_capacity(MAX_KEYFRAMES),
        }
    }

    pub fn try_interpolate(&self, target_time: f32) -> Option<T> {
        let mut interpolated_value = T::default();

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

                interpolated_value = interpolate_snapshots(start.value, end.value, t);

                return Some(interpolated_value);
            }
        }

        None
    }

    pub fn interpolate(&self, target_time: f32) -> T {
        //self.try_interpolate(target_time).unwrap_or_else(|| Vector3{x:1000.0, y:1000.0, z:1000.0})
        self.try_interpolate(target_time).unwrap_or_else(|| T::default())
    }

    pub fn push(&mut self, value: T, time: f32) {
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
    use crate::game_core::common::Vector3;
    use super::*;

    impl ViewSnapshot for Vector3 {
    }

    #[test]
    fn test_interpolation() {
        let mut buffer = BufferedViewSnapshotInterpolator::<Vector3>::new();

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
        assert_eq!(buffer.interpolate(3.5), Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_clear_before() {
        let mut buffer = BufferedViewSnapshotInterpolator::<Vector3>::new();

        buffer.push(Vector3::new(0.0, 0.0, 0.0), 0.0);
        buffer.push(Vector3::new(1.0, 0.0, 0.0), 1.0);
        buffer.push(Vector3::new(1.0, 1.0, 0.0), 2.0);
        buffer.push(Vector3::new(0.0, 1.0, 0.0), 3.0);

        buffer.clear_before(1.5);

        assert_eq!(buffer.interpolate(0.0), Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(buffer.interpolate(1.0), Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(buffer.interpolate(1.5), Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(buffer.interpolate(2.0), Vector3::new(1.0, 1.0, 0.0));
        assert_eq!(buffer.interpolate(3.0), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(buffer.interpolate(4.0), Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn debug(){
        let a = Vector3::new(0.0, 0.0, 0.0);
        println!("a: {:?}", a);
        let b = Vector3::new(1.0, 0.0, 0.0);
        println!("b: {:?}", b);
    }
}
