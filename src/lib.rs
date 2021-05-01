//! A procedural noise generation library for Rust.
//!
//! # Example
//!
//! ```rust
//! use noise::{NoiseFn, Perlin};
//!
//! let perlin = Perlin::new(1);
//! let val = perlin.get([42.4, 37.7, 2.8]);
//! ```

#![deny(missing_copy_implementations)]
#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]
#![feature(associated_type_defaults)]

#[doc(hidden)]
#[macro_use]
mod with_macro {
    macro_rules! with {
        ($vis:vis $property:ident: $type:ty) => {
            paste::paste! {
                $vis fn [<with_ $property>](self, $property: $type) -> Self {
                    Self { $property, ..self }
                }
            }
        };
    }
}

pub use crate::noise_fns::*;
pub use math::SamplePoint;

mod gradient;
mod math;
mod noise_fns;
mod permutationtable;
pub mod transforms;
pub mod utils;
