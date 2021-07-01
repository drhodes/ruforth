use crate::Data;
use crate::Data::{Float, Int};
// use types::Mach;

// fn binop<T>(lft: Data, rht: Data, op: &dyn Fn(T, T) -> Data ) -> Data{
//        match (lft, rht) {
//             (Int(x), Int(y)) => Int(x + y),
//             (Float(x), Float(y)) => Float(x + y),
//             (Float(x), Int(y)) => Float(x + y as f64),
//             (Int(x), Float(y)) => Float(x as f64 + y),
//         }
// }

impl std::ops::Add for Data {
    type Output = Self;

    fn add(self, other: Data) -> Data {
        match (self, other) {
            (Int(x), Int(y)) => Int(x + y),
            (Float(x), Float(y)) => Float(x + y),
            (Float(x), Int(y)) => Float(x + y as f64),
            (Int(x), Float(y)) => Float(x as f64 + y),
        }
    }
}

impl std::ops::Mul for Data {
    type Output = Self;

    fn mul(self, other: Data) -> Data {
        match (self, other) {
            (Int(x), Int(y)) => Int(x * y),
            (Float(x), Float(y)) => Float(x * y),
            (Float(x), Int(y)) => Float(x * y as f64),
            (Int(x), Float(y)) => Float(x as f64 * y),
        }
    }
}

impl std::ops::Sub for Data {
    type Output = Self;

    fn sub(self, other: Data) -> Data {
        match (self, other) {
            (Int(x), Int(y)) => Int(x - y),
            (Float(x), Float(y)) => Float(x - y),
            (Float(x), Int(y)) => Float(x - y as f64),
            (Int(x), Float(y)) => Float(x as f64 - y),
        }
    }
}

impl std::ops::Div for Data {
    type Output = Self;

    fn div(self, other: Data) -> Data {
        match (self, other) {
            (Int(x), Int(y)) => Int(x / y),
            (Float(x), Float(y)) => Float(x / y),
            (Float(x), Int(y)) => Float(x / y as f64),
            (Int(x), Float(y)) => Float(x as f64 / y),
        }
    }
}

