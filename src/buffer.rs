/*
 * @file tampon/buffer.rs
 *
 * @module tampon
 *
 * @brief Macro used to easily create a buffer from primitive, vectors and Tampon trait implementation.
 * 
 * @details
 * Macro used to easily create a buffer from primitive, vectors and Tampon trait implementation.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-07-16
 *
 * @version
 * 1.0 : 2022-07-16 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

/// ##### Variadic macro used to create a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer) and [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](macro.buffer.html#compatible-variabless). 
/// 
/// # Description
/// Variadic macro used to create a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer) and [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`bool`], [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) (except usize, isize), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// Also work with [`slice`] by using brackets `[]` instead of parenthesis `()`.
/// 
/// # Usage
/// `let buffer = buffer!([0..n](v1, ..., vn):type, [0..n][s1, ..., sn]:type);`
/// * One-to-many `(v1, ..., vn):type` where elements in `parenthesis()` are the variables to be copied into created buffer.
/// * One-to-many `[s1, ..., sn]:type` where elements in `brackets[]` are the slices to be copied into created buffer.
/// 
/// # Return
/// New buffer created with argument(s) serialized with the size needed to contain them all.
/// 
/// # Example(s)
/// ```
/// // Import macro buffer
/// use tampon::buffer;
/// 
/// // Declare multiple variables (numerics don't need to be same type)
/// let a:u8 = 55;
/// let b:u8 = 255;
/// let c:u32 = 12545566;
/// let d:String = String::from("Example string");
/// let e:Vec<i32> = vec![i32::MAX; 50];
/// let f:Vec<f64> = vec![f64::MAX; 50];
/// let g:Vec<f64> = vec![f64::MAX; 50];
/// 
/// // Create a buffer and serialize data with buffer! macro.
/// let buffer:Vec<u8> = buffer!((a,b):u8, (c):u32, (d):String, [e]:i32, [f,g]:f64);
/// 
/// // Print result
/// println!("Resulted buffer={:?}", buffer);
/// ```
/// 
/// # Compatible variables(s)
/// * [`bool`]
/// * All [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) except [`usize`] and [`isize`]
/// * [`String`] 
/// * Implementors of trait [`Tampon`](trait.Tampon.html)
/// * [`slice`] of the above types
/// 
/// # Endianness
/// * [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) bytes are written as [`little endian`](https://en.wikipedia.org/wiki/Endianness).
#[macro_export]
macro_rules! buffer {

    // Expression without tail
    (($expr:expr $(,$extra:expr)*):$type:ident) => {{
        // Get size needed for variable serialization
        let buffer_size = $crate::bytes_size!(($expr $(,$extra)*):$type);

        // Create mutable buffer of needed size
        let mut buffer:Vec<u8> = vec![0;buffer_size];

        // Serialize variable into vector
        $crate::serialize!(buffer, ($expr $(,$extra)*):$type);

        // Return buffer
        buffer
    } as Vec<u8> };

    // Expression with tail
    (($expr:expr $(,$extra:expr)*):$type:ident, $($tail:tt)*) => {{
        let buffer_size = $crate::bytes_size!(($expr $(,$extra)*):$type, $($tail)*);
        let mut buffer:Vec<u8> = vec![0;buffer_size];
        $crate::serialize!(buffer, ($expr $(,$extra)*):$type, $($tail)*);
        buffer
    } as Vec<u8> };

    // Slice without tail
    ([$expr:expr $(,$extra:expr)*]:$type:ident) => { {
        let buffer_size = $crate::bytes_size!([$expr $(,$extra)*]:$type);
        let mut buffer:Vec<u8> = vec![0;buffer_size];
        $crate::serialize!(buffer, [$expr $(,$extra)*]:$type);
        buffer
    } as Vec<u8> };

    // Slice with tail
    ([$expr:expr $(,$extra:expr)*]:$type:ident, $($tail:tt)*) => {{
        let buffer_size = $crate::bytes_size!([$expr $(,$extra)*]:$type, $($tail)*);
        let mut buffer:Vec<u8> = vec![0;buffer_size];
        $crate::serialize!(buffer, [$expr $(,$extra)*]:$type, $($tail)*);
        buffer
    } as Vec<u8> };
}