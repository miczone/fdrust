#![recursion_limit = "400"]

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "std")]
extern crate std;
#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod array;
pub mod helper;
pub mod algorithm;