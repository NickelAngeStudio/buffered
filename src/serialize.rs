/*
 * @file tampon/serialize.rs
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

/// ##### Variadic macro used to [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](macro.serialize.html#compatible-variabless) into a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer). 
/// 
/// # Description
/// Variadic macro used to [`serialize`](https://en.wikipedia.org/wiki/Serialization) [`bool`], [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) (except usize, isize), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// Also work with [`slice`] by using brackets `[]` instead of parenthesis `()`.
/// 
/// # Usage
/// `serialize!(buffer, [bytes_copied,] [0..n](v1, ..., vn):type, [0..n][s1, ..., sn]:type);`
/// * `buffer` - Mutable reference to [`slice`] of [`u8`] to copy bytes into.
/// * `bytes_copied` - (Optional) Identifier here can be used to get the count of bytes copied into buffer.
/// * One-to-many `(v1, ..., vn):type` where elements in `parenthesis()` are the variables to be copied into buffer.
/// * One-to-many `[s1, ..., sn]:type` where elements in `brackets[]` are the slices to be copied into buffer.
/// 
/// # Example(s)
/// ```
/// // Import macro bytes_size and serialize
/// use tampon::bytes_size;
/// use tampon::serialize;
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
/// // Get the size in bytes of all those elements in one macro call
/// let size = bytes_size!((a,b):u8, (c):u32, (d):String, [e]:i32, [f,g]:f64);
/// 
/// // Create a mutable buffer long enough to store variables
/// let mut buffer:Vec<u8> = vec![0;size];
/// 
/// // Serialize variables content into buffer
/// serialize!(buffer, bytes_copied, (a,b):u8, (c):u32, (d):String, [e]:i32, [f,g]:f64);
/// 
/// // (optional) Make sure bytes copied == bytes_size!
/// assert!(size == bytes_copied);
/// 
/// // Print result
/// println!("Bytes size of variables a,b,c,d,e,f is {}\nCopied size is {}\nResulted buffer={:?}", size, bytes_copied, buffer);
/// ```
/// ##### Buffer smaller than content will cause a panic! :
/// ``` should_panic
/// // Import macro bytes_size and serialize
/// use tampon::bytes_size;
/// use tampon::serialize;
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
/// // Mutable buffer TOO SMALL to contains variable
/// let mut buffer:Vec<u8> = vec![0;50];
/// 
/// // Copy variables content into buffer (will panic!)
/// serialize!(buffer, (a,b):u8, (c):u32, (d):String, [e]:i32, [f,g]:f64);
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
/// 
/// # Panic(s)
/// * Will panic! if `buffer` length is smaller than all sources length combined.
#[macro_export]
macro_rules! serialize {
    
    // Expression without tail without bytes_read
    ($buffer:expr,($expr:expr $(,$extra:expr)*):$type:ident) => { {
        let mut temporary_bytes_written = $crate::serialize_parser!($buffer, 0, ($expr $(,$extra)*):$type);
    }};

    // Expression with tail without bytes_read
    ($buffer:expr, ($expr:expr $(,$extra:expr)*):$type:ident, $($tail:tt)*) => {{
        let mut temporary_bytes_written = $crate::serialize_parser!($buffer, 0, ($expr $(,$extra)*):$type, $($tail)*);
    }};

    // Expression without tail with bytes_written
    ($buffer:expr, $bytes_written:ident, ($expr:expr $(,$extra:expr)*):$type:ident) => {
        // Dispatch to parser and get bytes_written
        let mut $bytes_written = $crate::serialize_parser!($buffer, 0, ($expr $(,$extra)*):$type);
    };

    // Expression with tail with bytes_written
    ($buffer:expr, $bytes_written:ident, ($expr:expr $(,$extra:expr)*):$type:ident, $($tail:tt)*) => {
        // Dispatch to parser and get bytes_written
        let mut $bytes_written = $crate::serialize_parser!($buffer, 0, ($expr $(,$extra)*):$type, $($tail)*);
    };


    // Slice without tail without bytes_read
    ($buffer:expr, [$expr:expr $(,$extra:expr)*]:$type:ident) => { {
        let mut temporary_bytes_written = $crate::serialize_parser!($buffer, 0, [$expr $(,$extra)*]:$type);
    }};

    // Slice with tail without bytes_read
    ($buffer:expr, [$expr:expr $(,$extra:expr)*]:$type:ident, $($tail:tt)*) => {{
        let mut temporary_bytes_written = $crate::serialize_parser!($buffer, 0, [$expr $(,$extra)*]:$type, $($tail)*);
    }};

    // Slice without tail with bytes_written
    ($buffer:expr, $bytes_written:ident, [$expr:expr $(,$extra:expr)*]:$type:ident) => {
        // Dispatch to parser and get bytes_written
        let mut $bytes_written = $crate::serialize_parser!($buffer, 0, [$expr $(,$extra)*]:$type);
    };

    // Slice with tail with bytes_written
    ($buffer:expr, $bytes_written:ident, [$expr:expr $(,$extra:expr)*]:$type:ident, $($tail:tt)*) => {
        // Dispatch to parser and get bytes_written
        let mut $bytes_written = $crate::serialize_parser!($buffer, 0, [$expr $(,$extra)*]:$type, $($tail)*);
    };


}

/// Hidden extension of the serialize! macro. Not meant to be used directly (although it will still work).
#[doc(hidden)]
#[macro_export]
macro_rules! serialize_parser {
    // Macro built with Incremental TT munchers pattern : https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html

    // Expression without tail
    ($buffer:expr, $index:expr, ($expr:expr $(,$extra:expr)*):$type:ident) => {{
        let buffer_size = $buffer.len();
        // Init bytes_copied with the expression
        let mut bytes_copied = $crate::serialize_retriever!($buffer[$index..buffer_size], $expr => $type);
        // Write extra to buffer and accumulate size
        $(bytes_copied += $crate::serialize_retriever!($buffer[$index + bytes_copied..buffer_size], $extra => $type); )*

        // Return bytes_copied
        bytes_copied
    } as usize };

    // Expression with tail
    ($buffer:expr, $index:expr, ($expr:expr $(,$extra:expr)*):$type:ident, $($tail:tt)*) => {{
        let buffer_size = $buffer.len();
        // Init bytes_copied with the expression
        let mut bytes_copied = $crate::serialize_retriever!($buffer[$index..buffer_size], $expr => $type);

        // Write extra to buffer and accumulate bytes_copied
        $(bytes_copied += $crate::serialize_retriever!($buffer[$index + bytes_copied..buffer_size], $extra => $type); )*

        // Write and accumulate tail TT
        bytes_copied += $crate::serialize_parser!($buffer, $index + bytes_copied, $($tail)*);

        // Return bytes_copied
        bytes_copied
    } as usize };

    // Slice without tail
    ($buffer:expr, $index:expr, [$expr:expr $(,$extra:expr)*]:$type:ident) => {{
        let buffer_size = $buffer.len();
        let mut bytes_copied = 0;

        // Get value from buffer into array
        bytes_copied += $crate::serialize_retriever!($buffer[$index + bytes_copied..buffer_size], $expr => [$type]);
        // Get value from buffer into array for extra
        $( bytes_copied += $crate::serialize_retriever!($buffer[$index + bytes_copied..buffer_size], $extra => [$type]); )*

        // Return bytes copied
        bytes_copied

    } as usize };

    // Slice with tail
    ($buffer:expr, $index:expr, [$expr:expr $(,$extra:expr)*]:$type:ident, $($tail:tt)*) => {{
        let buffer_size = $buffer.len();
        let mut bytes_copied = 0;

        // Get value from buffer into array
        bytes_copied += $crate::serialize_retriever!($buffer[$index + bytes_copied..buffer_size], $expr => [$type]);

        // Get value from buffer into array for extra
        $( bytes_copied += $crate::serialize_retriever!($buffer[$index + bytes_copied..buffer_size], $extra => [$type]); )*
        // Parse tail
        bytes_copied += $crate::serialize_parser!($buffer, $index + bytes_copied, $($tail)*);

        // Return bytes copied
        bytes_copied

    } as usize };

}


/// Hidden extension of the serialize! macro. Parse tokens. Not meant to be used directly (although it will still work).
#[doc(hidden)]
#[macro_export]
macro_rules! serialize_retriever {

    // Slice affectator
    ($buffer:expr, $expr:expr => [$type:ident]) => {{
        let buffer_size = $buffer.len();

        // Write size of slice
        let bytes_len = ($expr.len() as u32).to_le_bytes();
        $buffer[0..bytes_len.len()].copy_from_slice(&bytes_len);

        // Init bytes_copied at bytes_len.len() since we will loop slice
        let mut bytes_copied = bytes_len.len();

        // Loop and accumulate and element of slice
        for elem in $expr.iter() {
            bytes_copied += $crate::serialize_retriever!($buffer[bytes_copied..buffer_size], *elem => $type);
        } 

        // Return bytes_copied
        bytes_copied

    } as usize} ;


    /**********
    * BOOLEAN *
    **********/
    ($buffer:expr, $expr:expr => bool) => {{ 
        // Translate bytes into u8
        if($expr) {
            $crate::serialize_retriever!($buffer, 1 => u8)
        } else {
            $crate::serialize_retriever!($buffer, 0 => u8)
        }
        
    } as usize };


    /***********
    * NUMERICS * 
    ***********/
    ($buffer:expr, $expr:expr => u8) => {{ 
        // Transform the expression into bytes
        let bytes = <u8>::to_le_bytes($expr);

        // Copy to buffer via copy from slice
        $buffer[0..bytes.len()].copy_from_slice(&bytes);

        // Return size used
        bytes.len()
    } as usize };


    ($buffer:expr, $expr:expr => u16) => {{ 
        let bytes = <u16>::to_le_bytes($expr);
        $buffer[0..bytes.len()].copy_from_slice(&bytes);
        bytes.len()
    } as usize };


    ($buffer:expr, $expr:expr => u32) => {{ 
        let bytes = <u32>::to_le_bytes($expr);
        $buffer[0..bytes.len()].copy_from_slice(&bytes);
        bytes.len()
    } as usize };


    ($buffer:expr, $expr:expr => u64) => {{ 
        let bytes = <u64>::to_le_bytes($expr);
        $buffer[0..bytes.len()].copy_from_slice(&bytes);
        bytes.len()
    } as usize };


    ($buffer:expr, $expr:expr => u128) => {{ 
        let bytes = <u128>::to_le_bytes($expr);
        $buffer[0..bytes.len()].copy_from_slice(&bytes);
        bytes.len()
    } as usize };


    ($buffer:expr, $expr:expr => f32) => {{ 
        let bytes = <f32>::to_le_bytes($expr);
        $buffer[0..bytes.len()].copy_from_slice(&bytes);
        bytes.len()
    } as usize };

    ($buffer:expr, $expr:expr => f64) => {{ 
        let bytes = <f64>::to_le_bytes($expr);
        $buffer[0..bytes.len()].copy_from_slice(&bytes);
        bytes.len()
    } as usize };


    ($buffer:expr, $expr:expr => i8) => {{ 
        let bytes = <i8>::to_le_bytes($expr);
        $buffer[0..bytes.len()].copy_from_slice(&bytes);
        bytes.len()
    } as usize };


    ($buffer:expr, $expr:expr => i16) => {{ 
        let bytes = <i16>::to_le_bytes($expr);
        $buffer[0..bytes.len()].copy_from_slice(&bytes);
        bytes.len()
    } as usize };


    ($buffer:expr, $expr:expr => i32) => {{ 
        let bytes = <i32>::to_le_bytes($expr);
        $buffer[0..bytes.len()].copy_from_slice(&bytes);
        bytes.len()
    } as usize };


    ($buffer:expr, $expr:expr => i64) => {{ 
        let bytes = <i64>::to_le_bytes($expr);
        $buffer[0..bytes.len()].copy_from_slice(&bytes);
        bytes.len()
    } as usize };


    ($buffer:expr, $expr:expr => i128) => {{ 
        let bytes = <i128>::to_le_bytes($expr);
        $buffer[0..bytes.len()].copy_from_slice(&bytes);
        bytes.len()
    } as usize };

    /*********
    * STRING * 
    *********/
    ($buffer:expr, $expr:expr => String) => {{ 
        
        // Write size of String
        let bytes_size = ($expr.len() as u32).to_le_bytes();
        $buffer[0..bytes_size.len()].copy_from_slice(&bytes_size);

        // Transform String as bytes slice
        let bytes = $expr.as_bytes();

        // Copy to buffer via copy from slice
        $buffer[bytes_size.len()..(bytes_size.len() + bytes.len())].copy_from_slice(&bytes);

        // Return size used
        bytes_size.len() + bytes.len()

    } as usize };

    /***************
    * TAMPON TRAIT * 
    ***************/
    ($buffer:expr, $expr:expr => $tampon:ident) => {{
        $expr.serialize(&mut $buffer)
    } as usize };
}