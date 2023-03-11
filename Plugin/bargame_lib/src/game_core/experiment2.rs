use simba::scalar::FixedI40F24;

#[derive(Clone, Copy, Debug, PartialEq)]
struct MyFixedPoint(FixedI40F24);

impl MyFixedPoint {
    pub fn new(value: FixedI40F24) -> Self {
        MyFixedPoint(value)
    }

    pub fn inner(&self) -> FixedI40F24 {
        self.0
    }
}

impl Default for MyFixedPoint {
    fn default() -> Self {
        MyFixedPoint::new(FixedI40F24::from_num(0.0))
    }
}


impl From<MyFixedPoint> for FixedI40F24 {
    fn from(value: MyFixedPoint) -> Self {
        value.0
    }
}

impl From<FixedI40F24> for MyFixedPoint {
    fn from(value: FixedI40F24) -> Self {
        MyFixedPoint(value)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let num0 = MyFixedPoint::new(FixedI40F24::from_num(0.55));
        let default_num = MyFixedPoint::default();

        let num1 = num0.inner() + num0.inner();

        // Q: I want to be able to do math without having to call .inner() all the time, how can I do that?
        // A: Implement the From trait for MyFixedPoint and FixedI40F24



    }
}

