extern crate pest;
#[macro_use]
extern crate pest_derive;

extern crate nalgebra;
pub use nalgebra::{UnitQuaternion, Vector3};

mod types;
pub use types::{Attitude, Element, ElementType, Figure, RollType, Sequence};

pub mod olan;
