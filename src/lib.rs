 /// # Re-export for Public API
 #[doc(inline)]
 pub use generate::generate_buffer as generate_buffer;
 pub use generate::buffer_generator_charset as buffer_generator_charset;
 pub use wipe::wipe_buffer as wipe_buffer;
 pub use compare::compare_buffers as compare_buffers;

/// Generate buffer
#[doc(hidden)]
pub mod generate;

/// Wipe buffer
#[doc(hidden)]
pub mod wipe;

/// Compare buffers
#[doc(hidden)]
pub mod compare;

// Tests module folder
#[cfg(test)]
mod test;
