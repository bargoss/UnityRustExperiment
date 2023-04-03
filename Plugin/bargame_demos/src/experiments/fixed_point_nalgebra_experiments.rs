use simba::scalar::FixedI40F24;
use nalgebra::Vector2;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let num0 = FixedI40F24::from_num(0.5);
        let num1 = FixedI40F24::from_num(1.5);

        let vec0 = Vector2::new(num0, num1);

        let normalized_vec0 = vec0.normalize();
        let adsa = normalized_vec0 + normalized_vec0;

        println!("normalized_vec0: {:?}", normalized_vec0);
        println!("adsa: {:?}", adsa);

    }
}