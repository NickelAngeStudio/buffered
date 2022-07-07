/*
 * @file tampon/to_buffer.rs
 *
 * @module tampon
 *
 * @brief Macro used to easily fill a buffer from primitive, vectors and Tampon trait implementation.
 * 
 * @details
 * Macro used to easily fill a buffer from primitive, vectors and Tampon trait implementation.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-07-04
 *
 * @version
 * 1.0 : 2022-07-04 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

/// ##### Variadic macro used to fill a buffer with multiple primitives, slices and implementors of trait [`Tampon`](trait.Tampon.html). 
/// Variadic macro used to fill a buffer with multiple primitives, slices and implementors of trait [`Tampon`](trait.Tampon.html). 
/// 
/// # Argument(s)
/// * `buffer` - Buffer to copy into.
/// * `index` - Index of the buffer where to copy. 
/// * `<token>(var,type)` - 1..n variables to copy in buffer of according to `<token>` types :
///     * `p` - Copy [`primitive`](https://doc.rust-lang.org/rust-by-example/primitives.html) into buffer.
///     * `ps` - Copy [`slice`] of primitives into buffer.
///     * `t` - Copy implementors of [`Tampon`](trait.Tampon.html) trait into buffer.
///     * `ts` -Copy [`slice`] of implementors of [`Tampon`](trait.Tampon.html) trait into buffer.
/// 
/// # Return(s)
/// * Count of bytes written in buffer as [`usize`].
/// 
/// # Example(s)
/// ```
/// //TODO: Add example
/// ```
/// 
/// # Note(s)
/// * Works only with [`primitive`](https://doc.rust-lang.org/rust-by-example/primitives.html), [`slice`] of primitives
/// and implementors of trait [`Tampon`](trait.Tampon.html).
/// * Bytes are written as [`little endian`](https://en.wikipedia.org/wiki/Endianness).
/// 
/// # Panic(s)
/// * Will panic! if `buffer` length is smaller than all sources length combined.
#[macro_export]
macro_rules! to_buffer {

    // Numeric types
    ($buffer:expr, $index:expr, N($numeric:expr))=> {{
        
        // Get expression in slice of little endian bytes
        let src = $numeric.to_le_bytes();

        // Copy to buffer via copy from slice
        $buffer[$index..($index + src.len())].copy_from_slice(&src);

        // Return bytes count
        src.len()

    } as usize };

/*
    // Slice of numeric types
    ($buffer:expr, $index:expr, NS[$numerics:expr]) => {{

        // To accumulate bytes
        let mut bytes_size:usize = 0;

        // Take copy and get bytes size of each element
        for elem in $numerics.iter() {
            bytes_size += to_buffer!($buffer, $index + bytes_size, n (*elem));
        }

        // Return bytes count
        bytes_size

    } as usize };
*/

    // String
    ($buffer:expr, $index:expr, S($string:expr)) => {{
        // Return size of string (https://doc.rust-lang.org/std/string/struct.String.html#method.len-1)
        $string.len()
    } as usize };

    /*
    // Slice of strings
    ($buffer:expr, $index:expr, SS[$strings:expr]) => {{

        let mut bytes_size:usize = 0;

        // Take size of each string
        for elem in $strings.iter() {
            bytes_size += elem.len();
        }

        // Return size of strings
        bytes_size

    } as usize };
    */

    // Tampon trait implementor
    ($buffer:expr, $index:expr, T($tampon:expr)) => {{

        // Call Tampon::to_buffer with slice of buffer which return the size used.
        $tampon.to_buffer($buffer[$index..$buffer.len()])


    } as usize };

    /*
    // Slice of Tampon trait implementator
    ($buffer:expr, $index:expr, TS[$tampons:expr]) => {{

        // To keep the count of bytes used
        let mut bytes_size = 0;

        // Call tampon to_buffer impl for each element and increment the size used.
        for elem in $tampons.iter() {
            bytes_size += elem.to_buffer(&mut $buffer[$index + bytes_size..$buffer.len()]);
        }

        // Return bytes count
        bytes_size

    } as usize };
    */



    // Slice of numeric types
    ($buffer:expr, $index:expr, $token:tt[$slice:expr]) => {{

        // To accumulate bytes
        let mut bytes_size:usize = 0;

        // Take copy and get bytes size of each element
        for elem in $slice.iter() {
            bytes_size += to_buffer!($buffer, $index + bytes_size,$token(*elem));
        }

        // Return bytes count
        bytes_size

    } as usize };

    /*
    // Variadic pattern match
    ($buffer:expr, $index:expr, $token:tt $expr:expr, $($ext_token:tt $ext_expr:expr), *) => {{

        // Get non-variadic size
        let bytes_size = to_buffer!($buffer, $index, $token ($expr));

        // Return size used with variadic call
        bytes_size + to_buffer!($buffer, bytes_size + $index, $($ext_token $ext_expr ), *)

    } as usize };
    */

    /********************
    * VARIADIC PATTERNS *
    ********************/
    // Multiple elements in brackets []
    ($buffer:expr, $index:expr, $token:tt($expr:expr, $($var_expr:expr), *))=> {{

        // Get non-variadic size
        let bytes_size = to_buffer!($buffer, $index, $token($expr));

        // Return size used with variadic call
        bytes_size + to_buffer!($buffer, bytes_size + $index, $token($($var_expr),*))

    } as usize };

    // Multiple element in brackets [] and multiple tokens
    ($buffer:expr, $index:expr, $token:tt($($var_expr:expr), +), $($ext_token:tt($($etc_var_expr:expr), +)), *)=> {{

        // Get token variadic size
        let bytes_size = to_buffer!($buffer, $index, $token[$($var_expr),*]);

        // Return size used with variadic tokens
        bytes_size + to_buffer!($buffer, bytes_size + $index, $($ext_token[$($etc_var_expr),*]), *)

    } as usize };







    //////////////////////////////// OLD CODE BELOW


    /*
    // Non-variadic with only primitive
    ($buffer:expr, $index:expr, p ($primitive:expr, $type:ty)) => {{

        // Get expression in slice of little endian bytes
        let src = <$type>::to_le_bytes($primitive);

        // Copy to buffer via copy from slice
        $buffer[$index..($index + src.len())].copy_from_slice(&src);

        // Return bytes count
        $index + src.len()

    } as usize};

    // Non-variadic with slice of primitives
    ($buffer:expr, $index:expr, ps ($primitives:expr, $type:ty)) => {{

        // Get the size of the type in bytes
        let type_size = core::mem::size_of::<$type>();

        // Cycle each element and call to_buffer! macro
        for i in 0..$primitives.len() {
            to_buffer!($buffer, $index + (i * type_size), p ($buffer[i], $type));
        }

        // Return bytes count
        $index + type_size * $primitives.len()

    } as usize};

    // Non-variadic with trait Tampon implementator
    ($buffer:expr, $index:expr, t ($tampon:expr, $type:ty)) => {{
        // Call tampon to_buffer impl which return the size used.
        $tampon.to_buffer($buffer[$index..$buffer.len()])
    } as usize};

    // Non-variadic with slice of Tampon implementator
    ($buffer:expr, $index:expr, ts ($tampons:expr, $type:ty)) => {{

        // To keep the count of bytes used
        let mut bytes_size = 0;

        // Call tampon to_buffer impl for each element and increment the size used.
        for elem in $tampons.iter() {
            bytes_size += elem.to_buffer(&mut $buffer[$index..$buffer.len()]);
        }

        // Return bytes count
        bytes_size
    } as usize };

    // Variadic with primitive
    ($buffer:expr, $index:expr, p ($primitive:expr, $type:ty), $($token:tt ($ext_expr:expr, $ext_type:ty)), *) => {{

        // Get expression in slice of little endian bytes
        let src = <$type>::to_le_bytes($primitive);

        // Copy to buffer via copy from slice
        $buffer[$index..($index + src.len())].copy_from_slice(&src);

        // Return bytes count
        $index + src.len() + to_buffer!($buffer, $index + src.len(), $($token($ext_expr, $ext_type)), *)
    } as usize};

    // Variadic with slice of primitives
    ($buffer:expr, $index:expr, ps ($primitives:expr, $type:ty), $($token:tt ($ext_expr:expr, $ext_type:ty)), *) => {{

        // Get the size of the type in bytes
        let type_size = core::mem::size_of::<$type>();

        // Cycle each element and call to_buffer! macro
        for i in 0..$primitives.len() {
            to_buffer!($buffer, $index + (i * type_size), p ($buffer[i], $type));
        }

        // Return bytes count
        $index + type_size * $primitives.len() + to_buffer!($buffer, $index +  type_size * $primitives.len(), $($token($ext_expr, $ext_type)), *)
    } as usize};

    // Variadic with trait Tampon implementator
    ($buffer:expr, $index:expr, t ($tampon:expr, $type:ty), $($token:tt ($ext_expr:expr, $ext_type:ty)), *) => {{

        // Get size first
        let size = $tampon.to_buffer(&mut $buffer[$index..$buffer.len()]);

        // Recurse with size taken
        to_buffer!($buffer, $index +  size, $($token($ext_expr, $ext_type)), *)
    } as usize};

    // Variadic with slice of Tampon implementator
    ($buffer:expr, $index:expr, ts ($tampons:expr, $type:ty), $($token:tt ($ext_expr:expr, $ext_type:ty)), *) => {{

        // To keep the count of bytes used
        let mut bytes_size = 0;

        // Call tampon to_buffer impl for each element and increment the size used.
        for elem in tampons.iter() {
            bytes_size += elem.to_buffer($buffer[$index..$buffer.len()]);
        }

        // Return bytes count
        bytes_size + to_buffer!($buffer, $index + bytes_size, $($token($ext_expr, $ext_type)), *)
    } as usize};

    */
}


