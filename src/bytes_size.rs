/*
 * @file tampon/bytes_size.rs
 *
 * @module tampon
 *
 * @brief Contain variadic macro that return the byte size of elements.
 * 
 * @details
 * Contain variadic macro that return the byte size of elements.
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
 * 
 */

/// ##### Variadic macro used to get the size of bytes of [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html). 
/// 
/// Variadic macro used to get the size of bytes of [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// 
/// Also work with [`slice`] of those types.
///
/// # Argument(s)
/// * `<token>(var,type)` - 1..n variables to get size of according to `<token>` types :
///     * `n` - Count bytes of [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html).
///     * `ns` - Count bytes of [`slice`] of [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html).
///     * `s` - Count bytes of [`String`].
///     * `ss` - Count bytes of [`slice`] of [`String`].
///     * `t` - Count bytes of implementors of [`Tampon`](trait.Tampon.html) trait.
///     * `ts` - Count bytes of [`slice`] of implementors of [`Tampon`](trait.Tampon.html) trait.
/// 
/// # Example(s)
/// ```
/// //TODO: Add example
/// ```
/// 
/// # Note(s)
/// * Works only with [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// 
#[macro_export]
macro_rules! bytes_size {

    // Numeric types
    (n ($numeric:expr, $type:ty)) => {{
        // Return size of numeric
        core::mem::size_of::<$type>()
    } as usize };

    // Slice of numeric types
    (ns ($numerics:expr, $type:ty)) => {{
        // Return size of numerics
        core::mem::size_of::<$type>() * $numerics.len()
    } as usize };

    // String
    (s ($string:expr, $type:ty)) => {{
        // Return size of string (https://doc.rust-lang.org/std/string/struct.String.html#method.len-1)
        $string.len()
    } as usize };

    // Slice of strings
    (ss ($strings:expr, $type:ty)) => {{

        let mut bytes_size:usize = 0;

        // Take size of each string
        for elem in $strings.iter() {
            bytes_size += elem.len();
        }

        // Return size of strings
        bytes_size

    } as usize };

    // Tampon trait implementor
    (t ($tampon:expr, $type:ty)) => {{
        // Return size of trait
        $tampon.bytes_size()
    } as usize };

    // Slice of Tampon trait implementator
    (ts ($tampons:expr, $type:ty)) => {{

        if $tampons.len() > 0 {
            $tampons[0].bytes_size() * $tampons.len()
        } 
        else {
            // Return 0 if no elements in traits slice
            0
        }
    } as usize };

    // Variadic pattern match
    ($token:tt ($expr:expr, $type:ty), $($ext_token:tt ($ext_expr:expr, $ext_type:ty)), *) => {{
        bytes_size!($token ($expr, $type)) + bytes_size!($($ext_token($ext_expr, $ext_type)), *)
    } as usize };
}