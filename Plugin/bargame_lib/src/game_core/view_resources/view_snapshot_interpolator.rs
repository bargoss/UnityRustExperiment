use std::collections::{HashMap, VecDeque};
use crate::game_core::common::*;

use crate::game_core::view_resources::view_snapshot::{interpolate_snapshots, ViewSnapshot};
use bevy_ecs::prelude::Resource;

use crate::game_core::math::*;

pub struct InterpolationKeyFrame<T> where T: ViewSnapshot{
    pub value: T,
    pub time: FP,
}
const MAX_KEYFRAMES: usize = 8;

#[derive(Default)]
struct BufferedViewSnapshotInterpolatorItem<T> where T: ViewSnapshot {
    key_frames: VecDeque<InterpolationKeyFrame<T>>, //todo: make it into a fixed size array
}


impl <T> BufferedViewSnapshotInterpolatorItem<T> where T: ViewSnapshot {
    pub fn try_interpolate(&self, target_time: FP) -> Option<T> {
        let interpolated_value;

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
                // print params and result
                //println!("start: {:?}, end: {:?}, t: {:?}, result: {:?}", start.value, end.value, t, interpolated_value);

                return Some(interpolated_value);
            }
        }

        None
    }

    pub fn interpolate(&self, target_time: FP) -> T {
        self.try_interpolate(target_time).unwrap_or_else(|| T::default())
    }

    pub fn push(&mut self, value: T, time: FP) {
        self.key_frames.push_back(InterpolationKeyFrame { value, time });
        if self.key_frames.len() > MAX_KEYFRAMES {
            self.key_frames.pop_front();
        }
    }

    pub fn clear_before(&mut self, time: FP) {
        if self.key_frames.iter().any(|frame| frame.time < FP::zero()) {
            eprintln!("Keyframe time is negative");
        }
        self.key_frames.retain(|frame| frame.time >= time);
    }
}

#[derive(Resource, Default)]
pub struct BufferedViewSnapshotInterpolator<T> where T: ViewSnapshot {
    items: HashMap<Id, BufferedViewSnapshotInterpolatorItem<T>>,
}

impl <T> BufferedViewSnapshotInterpolator<T> where T: ViewSnapshot {
    pub fn try_interpolate(&self, id: Id, target_time: FP) -> Option<T> {
        self.items.get(&id).and_then(|item| item.try_interpolate(target_time))
    }

    pub fn interpolate(&self, id: Id, target_time: FP) -> T {
        self.try_interpolate(id, target_time).unwrap_or_else(|| T::default())
    }

    pub fn push(&mut self, view_custom_id: Id, time: FP, value: T) {
        self.items.entry(view_custom_id).or_insert_with(BufferedViewSnapshotInterpolatorItem::default).push(value, time);
    }

    pub fn clear_before(&mut self, time: FP) {
        for item in self.items.values_mut() {
            item.clear_before(time);
        }
    }
    // iterate
    //pub fn iter(&self) -> impl Iterator<Item = (&Id, &BufferedViewSnapshotInterpolatorItem<T>)> {
    //    self.items.iter()
    //}

    pub fn interpolated_keyframes(
        &self,
        view_time: FP,
    ) -> impl Iterator<Item = (Id, T)> + '_ {
        self.items.iter().map(move |(id, item)| {
            //println!("interpolating {} at {}", id.0, view_time);
            let interpolated_value = item.interpolate(view_time);
            //println!("interpolated value: {:?}", interpolated_value);
            (*id, interpolated_value)
        })
    }
}


#[cfg(test)]
mod tests {
    
    use super::*;

    impl ViewSnapshot for FP3 {
    }

    #[test]
    fn test_interpolation() {
        let mut buffer = BufferedViewSnapshotInterpolatorItem::<FP3>::default();

        buffer.push(FP3::from_num(0.0, 0.0, 0.0), FP::new(0.0));
        buffer.push(FP3::from_num(1.0, 0.0, 0.0), FP::new(1.0));
        buffer.push(FP3::from_num(1.0, 1.0, 0.0), FP::new(2.0));
        buffer.push(FP3::from_num(0.0, 1.0, 0.0), FP::new(3.0));

        assert_eq!(buffer.interpolate(FP::new(0.0)), FP3::from_num(0.0, 0.0, 0.0));
        assert_eq!(buffer.interpolate(FP::new(0.5)), FP3::from_num(0.5, 0.0, 0.0));
        assert_eq!(buffer.interpolate(FP::new(1.0)), FP3::from_num(1.0, 0.0, 0.0));
        assert_eq!(buffer.interpolate(FP::new(1.5)), FP3::from_num(1.0, 0.5, 0.0));
        assert_eq!(buffer.interpolate(FP::new(2.0)), FP3::from_num(1.0, 1.0, 0.0));
        assert_eq!(buffer.interpolate(FP::new(2.5)), FP3::from_num(0.5, 1.0, 0.0));
        assert_eq!(buffer.interpolate(FP::new(3.0)), FP3::from_num(0.0, 1.0, 0.0));
        assert_eq!(buffer.interpolate(FP::new(3.5)), FP3::from_num(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_clear_before() {
        let mut buffer = BufferedViewSnapshotInterpolatorItem::<FP3>::default();

        buffer.push(FP3::from_num(0.0, 0.0, 0.0), FP::new(0.0));
        buffer.push(FP3::from_num(1.0, 0.0, 0.0), FP::new(1.0));
        buffer.push(FP3::from_num(1.0, 1.0, 0.0), FP::new(2.0));
        buffer.push(FP3::from_num(0.0, 1.0, 0.0), FP::new(3.0));

        buffer.clear_before(FP::new(1.5));

        assert_eq!(buffer.interpolate(FP::new(0.0)), FP3::from_num(0.0, 0.0, 0.0));
        assert_eq!(buffer.interpolate(FP::new(1.0)), FP3::from_num(0.0, 0.0, 0.0));
        assert_eq!(buffer.interpolate(FP::new(1.5)), FP3::from_num(0.0, 0.0, 0.0));
        assert_eq!(buffer.interpolate(FP::new(2.0)), FP3::from_num(1.0, 1.0, 0.0));
        assert_eq!(buffer.interpolate(FP::new(3.0)), FP3::from_num(0.0, 1.0, 0.0));
        assert_eq!(buffer.interpolate(FP::new(4.0)), FP3::from_num(0.0, 0.0, 0.0));
    }

    #[test]
    fn debug(){
        let a = FP3::from_num(0.0, 0.0, 0.0);
        println!("a: {:?}", a);
        let b = FP3::from_num(1.0, 0.0, 0.0);
        println!("b: {:?}", b);
    }
}
