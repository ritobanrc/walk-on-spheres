#[allow(dead_code)]
mod vec_ext;

pub use vec_ext::VecExtPartialOrd;

use nalgebra as na;
pub const DIM: usize = 2;

pub type Dim = na::Const<DIM>;

pub type Real = f64;
pub type Vector = na::SVector<Real, DIM>;
//pub type IntVector = na::SVector<isize, DIM>;
pub type UIntVector = na::SVector<usize, DIM>;

pub use std::f64::consts;

//pub type Mat = na::SMatrix<Real, DIM, DIM>;
