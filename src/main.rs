extern crate nalgebra;

use nalgebra::{U3, Matrix, ArrayStorage};

type Matrix3x3f = Matrix<f32, U3, U3, ArrayStorage<f32, U3, U3>>;

fn main() {
    let m = Matrix3x3f::new(11.0, 12.0, 13.0,
                            21.0, 22.0, 23.0,
                            31.0, 32.0, 33.0);
    println!("{:?}", 2.0*m);
}
