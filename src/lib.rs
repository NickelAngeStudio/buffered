#![doc(html_playground_url = "https://play.rust-lang.org/")]

// TODO: Add logo
//#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/67743099?v=4")]

//! Crate that contains [`SAFE`](https://doc.rust-lang.org/nomicon/meet-safe-and-unsafe.html) 
//! Rust [`functions`](https://doc.rust-lang.org/rust-by-example/fn.html), 
//! [`macro`](https://doc.rust-lang.org/rust-by-example/macros.html) 
//! and [`trait`](https://doc.rust-lang.org/rust-by-example/trait.html) to generate and manage [`buffer`](https://en.wikipedia.org/wiki/Data_buffer).

 /// # Re-export for Public API
 #[doc(inline)]
 pub use generate::generate_buffer as generate_buffer;
 pub use generate::buffer_generator_charset as buffer_generator_charset;
 pub use wipe::wipe_buffer as wipe_buffer;
 pub use compare::compare_buffers as compare_buffers;
 pub use crate::tampon::Tampon as Tampon;
 pub use bytes_size::SLICE_SIZE_IN_BYTES as SLICE_SIZE_IN_BYTES;

/// Generate buffer
#[doc(hidden)]
pub mod generate;

/// Wipe buffer
#[doc(hidden)]
pub mod wipe;

/// Compare buffers
#[doc(hidden)]
pub mod compare;

/// Tampon trait
#[doc(hidden)]
pub mod tampon;

/// bytes_size macro
#[doc(hidden)]
pub mod bytes_size;

/// to_buffer macro
#[doc(hidden)]
pub mod to_buffer;

/// from_buffer macro
#[doc(hidden)]
pub mod from_buffer;


// Tests module folder
#[cfg(test)]
mod test;
