use serde::{Serialize, Deserialize};

use super::super::game_core::math::FixedPoint;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: FixedPoint,
    y: FixedPoint
}

#[derive(Serialize, Deserialize, Debug)]
struct Line {
    points: Vec<Point>,
    valid: bool,
    length: f32,
    desc: String
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_and_deserialize_example() {
        let point1: Point = Point {x:FixedPoint::new(1.0), y:FixedPoint::new(2.0)};
        let point2: Point = Point {x:FixedPoint::new(3.0), y:FixedPoint::new(4.0)};
        let point1s = bincode::serialize(&point1).unwrap();
        let point2s = bincode::serialize(&point2).unwrap();
        println!("struct Point serializes into byte array {:?}", point1s);
        println!("struct Point serializes into byte array {:?}", point2s);

        let length = ((point2.x - point1.x).to_f32() * (point2.x - point1.x).to_f32() + (point2.y - point1.y).to_f32() * (point2.y - point1.y).to_f32()).sqrt();
        let valid = if length == 0.0 { false } else { true };
        let line = Line { points: vec![point1, point2], valid: valid, length: length, desc: "a thin line".to_string() };
        let lines = bincode::serialize(&line).unwrap();
        println!("struct Line serializes into byte array {:?}", lines);

        let lined: Line = bincode::deserialize(&lines).unwrap();
        assert_eq!(lined.desc, "a thin line");
        assert_eq!(lined.length, 2.828427);
    }

    #[test]
    fn write_to_buffer_sequentially_example(){
        // define the buffer
        let _buffer: Vec<u8> = Vec::new();

        // define the data
        let point1: Point = Point {x:FixedPoint::new(1.0), y:FixedPoint::new(2.0)};
        let point2: Point = Point {x:FixedPoint::new(3.0), y:FixedPoint::new(4.0)};

        // serialize the data
        let _point1s = bincode::serialize(&point1).unwrap();
        let _point2s = bincode::serialize(&point2).unwrap();

        /*

         */
    }

}