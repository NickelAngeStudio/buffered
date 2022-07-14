/*
 * @file tampon/tampon.rs
 *
 * @module tampon
 *
 * @brief Contain Tampon trait used in macros.
 * 
 * @details
 * Contain Tampon trait used in macros.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-07-02
 *
 * @version
 * 1.0 : 2022-07-02 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 * // TODO : Update doc macro links and description
 */

/// ##### Trait used to make it's implementors compatible with tampon macros.
/// This trait must be implemented by object that want to be able to use the [`token`](https://doc.rust-lang.org/reference/tokens.html) trait of tampon macros.
pub trait Tampon<T> {
    /// Size of the trait implementation in bytes.
    /// 
    /// Used by [`bytes_size!`] macro.
    fn bytes_size(&self) -> usize;

    /// Write the trait implementation variables into the buffer and return bytes written. 
    /// 
    /// Used by [`to_buffer!`] macro.
    /// # Argument(s)
    /// * `buffer` - Mutable slice reference to buffer to write into. 
    /// # Return
    /// Size of bytes written into buffer.
    fn to_buffer(&self, buffer : &mut [u8]) -> usize;

    /// Create the implementation variables values from the buffer and return it with bytes read.
    /// 
    /// Used by [`from_buffer!`] macro.
    /// # Argument(s)
    /// * `buffer` - Slice reference to buffer to read from. 
    /// # Return
    /// Tuple of new object and size of bytes read from buffer.
    fn from_buffer(buffer : &[u8]) -> (T, usize);
}