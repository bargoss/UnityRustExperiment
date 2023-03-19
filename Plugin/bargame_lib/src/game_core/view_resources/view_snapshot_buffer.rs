use crate::game_core::view_resources::view_snapshot::ViewSnapshot;

pub struct Keyframe<T> where T : ViewSnapshot{
    pub time: f32,
    pub snapshot: T,
}

const MAX_KEYFRAMES: usize = 8;


pub struct ViewSnapshotBuffer<T> where T : ViewSnapshot{
    keyframes: Vec<Keyframe<T>>,
}

impl<T> ViewSnapshotBuffer<T> where T : ViewSnapshot{
    pub fn new() -> ViewSnapshotBuffer<T>{
        ViewSnapshotBuffer{
            keyframes: Vec::with_capacity(MAX_KEYFRAMES),
        }
    }

    pub fn add_keyframe(&mut self, keyframe: Keyframe<T>){
        self.keyframes.push(keyframe);
    }

    pub fn get_keyframes(&self) -> &Vec<Keyframe<T>>{
        &self.keyframes
    }

    pub fn clear(&mut self){
        self.keyframes.clear();
    }
}

