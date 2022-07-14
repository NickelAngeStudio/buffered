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

/// For compatibility between [`32-bit and 64-bit`](https://www.geeksforgeeks.org/difference-32-bit-64-bit-operating-systems/) architectures,
/// the maximum [`slice`] size allowed is [`u32::MAX`](https://doc.rust-lang.org/std/primitive.u32.html#associatedconstant.MAX)
/// which take 4 bytes.
pub const SLICE_SIZE_IN_BYTES:usize = 4;

/// ##### Variadic macro used to get the size of bytes of [`variables`](macro.bytes_size.html#compatible-variabless) and implementors of trait [`Tampon`](trait.Tampon.html). 
/// 
/// Variadic macro used to get the size of bytes of [`bool`], [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) (except usize, isize), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// Also work with [`slice`] by using brackets `[]` instead of parenthesis `()`.
///
/// # Usage
/// `let size = bytes_size!([0..n](v1, ..., vn):type, [0..n][s1, ..., sn]:type);`
/// * One-to-many `(v1, ..., vn):type` where elements in `parenthesis()` are the variables to be sized.
/// * One-to-many `[s1, ..., sn]:type` where elements in `brackets[]` are the slices to be sized.
/// 
/// # Return
/// Size in bytes of all arguments as [`usize`].
/// 
/// # Example(s)
/// ```
/// // Import macro
/// use tampon::bytes_size;
/// 
/// // Declare multiple variables
/// let a:u8 = 55;
/// let b:u8 = 255;
/// let c:u32 = 12545566;
/// let d:String = String::from("Example string");
/// let e:Vec<i32> = vec![i32::MAX; 50];
/// let f:Vec<f64> = vec![f64::MAX; 50];
/// let g:Vec<f64> = vec![f64::MAX; 50];
/// 
/// // Get the size in bytes of all those elements in one macro call
/// let size = bytes_size!((a,b):u8, (c):u32, (d):String, [e]:i32, [f,g]:f64);
/// 
/// // Print result
/// println!("Bytes size of variables a,b,c,d,e,f,g is {}", size);
/// ```
/// 
/// # Compatible variables(s)
/// * [`bool`]
/// * All [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) except [`usize`] and [`isize`]
/// * [`String`] 
/// * Implementors of trait [`Tampon`](trait.Tampon.html)
/// * [`slice`] of the above types
/// 
#[macro_export]
macro_rules! bytes_size {
    // Macro built with Incremental TT munchers pattern : https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html

    // Return 0 on empty
    () => {{ 0 } as usize };

    // Without tail
    (($expr:expr $(,$extra:expr)*):$type:ident) => {{
        $crate::bytes_size_var!($expr => $type) $(+$crate::bytes_size_var!($extra => $type))*
    } as usize };

    // With tail
    (($expr:expr $(,$extra:expr)*):$type:ident, $($tail:tt)*) => {{
        bytes_size!($($tail)*) + $crate::bytes_size_var!($expr => $type) $(+$crate::bytes_size_var!($extra => $type))*
    } as usize };

    // Without tail
    ([$expr:expr $(,$extra:expr)*]:$type:ident) => {{
        $crate::bytes_size_var!($expr => [$type]) $(+$crate::bytes_size_var!($extra => [$type]))*
    } as usize };

    // With tail
    ([$expr:expr $(,$extra:expr)*]:$type:ident, $($tail:tt)*) => {{
        bytes_size!($($tail)*)  + $crate::bytes_size_var!($expr => [$type]) $(+$crate::bytes_size_var!($extra => [$type]))*
    } as usize };
}

/// Hidden extension of the bytes_size! macro. Not meant to be used directly (although it will still work).
#[doc(hidden)]
#[macro_export]
macro_rules! bytes_size_var {
    /**********
    * BOOLEAN *
    **********/
    ($expr:expr => bool) => {{ 
        // bool use 1 byte (even if 1 bit)
        1 
    } as usize };
    ($expr:expr => [bool]) => {{ 
        // Size padding + length of slice
        $crate::SLICE_SIZE_IN_BYTES + $expr.len() 
    } as usize };

    /***********
    * NUMERICS * 
    ***********/
    ($expr:expr => u8) => {{ 
        // Size of type in bytes
        core::mem::size_of::<u8>() 
    } as usize };
    ($expr:expr => [u8]) => {{ 
        // Size padding + size of type * length of slice
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<u8>() * $expr.len() 
    } as usize };

    ($expr:expr => u16) => {{ 
        core::mem::size_of::<u16>() 
    } as usize };
    ($expr:expr => [u16]) => {{ 
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<u16>() * $expr.len() 
    } as usize };

    ($expr:expr => u32) => {{ 
        core::mem::size_of::<u32>() 
    } as usize };
    ($expr:expr => [u32]) => {{ 
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<u32>() * $expr.len() 
    } as usize };

    ($expr:expr => u64) => {{ 
        core::mem::size_of::<u64>() 
    } as usize };
    ($expr:expr => [u64]) => {{ 
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<u64>() * $expr.len() 
    } as usize };

    ($expr:expr => u128) => {{ 
        core::mem::size_of::<u128>() 
    } as usize };
    ($expr:expr => [u128]) => {{ 
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<u128>() * $expr.len() 
    } as usize };

    ($expr:expr => f32) => {{ 
        core::mem::size_of::<f32>() 
    } as usize };
    ($expr:expr => [f32]) => {{ 
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<f32>() * $expr.len() 
    } as usize };

    ($expr:expr => f64) => {{ 
        core::mem::size_of::<f64>() 
    } as usize };
    ($expr:expr => [f64]) => {{ 
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<f64>() * $expr.len() 
    } as usize };

    ($expr:expr => i8) => {{ 
        core::mem::size_of::<i8>() 
    } as usize };
    ($expr:expr => [i8]) => {{ 
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<i8>() * $expr.len() 
    } as usize };

    ($expr:expr => i16) => {{ 
        core::mem::size_of::<i16>() 
    } as usize };
    ($expr:expr => [i16]) => {{ 
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<i16>() * $expr.len() 
    } as usize };

    ($expr:expr => i32) => {{ 
        core::mem::size_of::<i32>() 
    } as usize };
    ($expr:expr => [i32]) => {{ 
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<i32>() * $expr.len() 
    } as usize };

    ($expr:expr => i64) => {{ 
        core::mem::size_of::<i64>() 
    } as usize };
    ($expr:expr => [i64]) => {{ 
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<i64>() * $expr.len() 
    } as usize };

    ($expr:expr => i128) => {{ 
        core::mem::size_of::<i128>() 
    } as usize };
    ($expr:expr => [i128]) => {{ 
        $crate::SLICE_SIZE_IN_BYTES + core::mem::size_of::<i128>() * $expr.len() 
    } as usize };


    /*********
    * STRING * 
    *********/
    ($expr:expr => String) => {{ 
        // String is a slice of char and need to pad the size
        // String::len() gives size of string in bytes (https://doc.rust-lang.org/std/string/struct.String.html#method.len-1)
        $crate::SLICE_SIZE_IN_BYTES + $expr.len()
    } as usize };
    ($expr:expr => [String]) => {{

        // We have to iterate [String] since each String can have a different length
        let mut bytes_size = 0;
        for elem in $expr.iter() {
            bytes_size += $crate::SLICE_SIZE_IN_BYTES + elem.len()
        }
        $crate::SLICE_SIZE_IN_BYTES + bytes_size

    } as usize };


    /***************
    * TAMPON TRAIT * 
    ***************/
    ($expr:expr => $tampon:ident) => {{
        $expr.bytes_size()
    } as usize };
    ($expr:expr => [$tampon:ident]) => {{ 
        // We have to iterate [Tampon] since each tampon can have a different size
        let mut bytes_size = 0;
        for elem in $expr.iter() {
            bytes_size += elem.bytes_size();
        }
        $crate::SLICE_SIZE_IN_BYTES + bytes_size
    } as usize };

}