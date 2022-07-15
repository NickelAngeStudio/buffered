/*
 * @file tampon/deserialize.rs
 *
 * @module tampon
 *
 * @brief Macro used to easily retrieve values from buffer to primitive, vectors and Tampon trait implementation.
 * 
 * @details
 * Macro used to easily retrieve values from buffer to primitive, vectors and Tampon trait implementation.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-07-13
 *
 * @version
 * 1.0 : 2022-07-13 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

/// ##### Variadic macro used to [`deserialize`](https://en.wikipedia.org/wiki/Serialization) [`compatible variables`](macro.deserialize.html#compatible-variabless) from a [`buffer`](https://en.wikipedia.org/wiki/Data_buffer). 
/// 
/// # Description
/// Variadic macro used to [`deserialize`](https://en.wikipedia.org/wiki/Serialization) [`bool`], [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) (except usize, isize), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// Also work with [`slice`] by using brackets `[]` instead of parenthesis `()`.
/// 
/// <b>deserialize! automatically creates variables when retrieving data.</b>
/// # Usage
/// `deserialize!(buffer, [bytes_read,] [0..n](v1, ..., vn):type, [0..n][s1, ..., sn]:type);`
/// * `buffer` - Unmutable reference to [`slice`] of [`u8`] to copy bytes from.
/// * `bytes_read` - (Optional) Identifier here can be used to get the count of bytes read from buffer.
/// * One-to-many `(v1, ..., vn):type` where elements in `parenthesis()` are the variables to be read from buffer.
/// * One-to-many `[s1, ..., sn]:type` where elements in `brackets[]` are the slices to be read from buffer.
/// 
/// # Example(s)
/// ```
/// // Import macro deserialize
/// use tampon::deserialize;
/// 
/// // Any buffer made with to_buffer! with (4554):u16, (65598.48896):f64, (c=12545566,d=456878):u32, ("Example string"):String, [vec![i32::MAX; 5]]:i32
/// let buffer: Vec<u8> = vec![202, 17, 145, 184, 199, 210, 231, 3, 240, 64, 30, 110, 191, 0, 174, 248, 6, 0, 14, 0, 0, 0, 69, 120, 97, 109, 112, 108, 101, 32, 115, 116, 114, 105, 110, 103, 5, 0, 0, 0, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127, 255, 255, 255, 127];
///
/// // Deserialize data from buffer. (variable are created during deserialization process)
/// deserialize!(buffer, bytes_read, (a):u16, (b):f64, (c,d):u32, (e):String, [f]:i32);
///
/// // Print result
/// println!("Bytes read={} | a={}, b={}, c={}, d={}, e={}, f={:?}", bytes_read, a,b,c,d,e,f);
/// ```
/// ##### Buffer smaller than content to retrieve will cause a panic! :
/// ``` should_panic
/// // Import macro deserialize
/// use tampon::deserialize;
/// 
/// // Any buffer made too small for data to retrieve.
/// let buffer: Vec<u8> = vec![202, 17, 145, 184, 199, 210, 231, 3, 240, 64, 30, 110, 191, 0, 174, 248, 6, 0, 14, 0, 0, 0, 69, 120, 97, 109, 112, 108, 101, 32, 115, 116, 114, 105, 110];
///
/// // Deserialize data from buffer (will panic!)
/// deserialize!(buffer, bytes_read, (_a):u16, (_b):f64, (_c,_d):u32, (_e):String, [_f]:i32);
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
/// * Will panic! if `buffer` length is smaller than all target length combined.
#[macro_export]
macro_rules! deserialize {
    /************
    * VARIABLES * 
    ************/
     // Expression without tail without bytes_read
     ($buffer:expr, ($name:ident $(,$extra:ident)*):$type:ident) => {
        let mut temporary_bytes_read = 0;
        $crate::deserialize_parser!($buffer, 0, temporary_bytes_read, ($name $(,$extra)*):$type);
    };

    // Expression with tail without bytes_read
    ($buffer:expr, ($name:ident $(,$extra:ident)*):$type:ident, $($tail:tt)*) => {
        let mut temporary_bytes_read = 0;
        $crate::deserialize_parser!($buffer, 0, temporary_bytes_read, ($name $(,$extra)*):$type, $($tail)*);
    };

    // Expression without tail with bytes_read
    ($buffer:expr, $bytes_read:ident, ($name:ident $(,$extra:ident)*):$type:ident) => {
        // Initialize bytes_read token
        let mut $bytes_read = 0;
        // Send to deserialize_parser
        $crate::deserialize_parser!($buffer, 0, $bytes_read, ($name $(,$extra)*):$type);
    };
    
    // Expression with tail with bytes_read
    ($buffer:expr, $bytes_read:ident, ($name:ident $(,$extra:ident)*):$type:ident, $($tail:tt)*) => {
        // Initialize bytes_read token
        let mut $bytes_read = 0;
        // Send to deserialize_parser
        $crate::deserialize_parser!($buffer, 0, $bytes_read, ($name $(,$extra)*):$type, $($tail)*);
    };


    /*********
    * SLICES * 
    *********/
    // SLICE Without tail without bytes_read
    ($buffer:expr, [$name:ident $(,$extra:ident)*]:$type:ident) => {
        let mut temporary_bytes_read = 0;
        $crate::deserialize_parser!($buffer, 0, temporary_bytes_read, [$name $(,$extra)*]:$type);
    };

    // SLICE With tail without bytes_read
    ($buffer:expr, [$name:ident $(,$extra:ident)*]:$type:ident, $($tail:tt)*) => {
        let mut temporary_bytes_read = 0;
        $crate::deserialize_parser!($buffer, 0, temporary_bytes_read, [$name $(,$extra)*]:$type, $($tail)*);
    };

    // SLICE Without tail with bytes_read
    ($buffer:expr, $bytes_read:ident, [$name:ident $(,$extra:ident)*]:$type:ident) => {
        // Initialize bytes_read token
        let mut $bytes_read = 0;
        // Send to deserialize_parser
        $crate::deserialize_parser!($buffer, 0, $bytes_read, [$name $(,$extra)*]:$type);
    };

    // SLICE With tail with bytes_read
    ($buffer:expr, $bytes_read:ident, [$name:ident $(,$extra:ident)*]:$type:ident, $($tail:tt)*) => {
        // Initialize bytes_read token
        let mut $bytes_read = 0;
        // Send to deserialize_parser
        $crate::deserialize_parser!($buffer, 0, $bytes_read, [$name $(,$extra)*]:$type, $($tail)*);
    };
    
}

/// Hidden extension of the to_buffer! macro. Parse tokens. Not meant to be used directly (although it will still work).
#[doc(hidden)]
#[macro_export]
macro_rules! deserialize_parser {
    // Macro built with Incremental TT munchers pattern : https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html

    // Expression without tail with bytes_read
    ($buffer:expr, $index:expr, $bytes_read:expr, ($name:ident $(,$extra:ident)*):$type:ident) => {
        // Get value from buffer into expression
        $crate::deserialize_retriever!($bytes_read, $buffer[$index + $bytes_read..$buffer.len()], $name => $type);
        // Get value from buffer into expression for extra
        $( $crate::deserialize_retriever!($bytes_read, $buffer[$index + $bytes_read..$buffer.len()], $extra => $type); )*
    };

    // Expression with tail with bytes_read
    ($buffer:expr, $index:expr, $bytes_read:expr, ($name:ident $(,$extra:ident)*):$type:ident, $($tail:tt)*) => {
        // Get value from buffer into expression
        $crate::deserialize_retriever!($bytes_read, $buffer[$index + $bytes_read..$buffer.len()], $name => $type);
        // Get value from buffer into expression for extra
        $( $crate::deserialize_retriever!($bytes_read, $buffer[$index + $bytes_read..$buffer.len()], $extra => $type); )*
        // Parse tail
        $crate::deserialize_parser!($buffer, $index, $bytes_read, $($tail)*);
    };

    // SLICE Without tail with bytes_read
    ($buffer:expr, $index:expr, $bytes_read:expr, [$name:ident $(,$extra:ident)*]:$type:ident) => {
        // Get value from buffer into array
        $crate::deserialize_retriever!($bytes_read, $buffer[$index + $bytes_read..$buffer.len()], $name => [$type]);
        // Get value from buffer into array for extra
        $( $crate::deserialize_retriever!($bytes_read, $buffer[$index + $bytes_read..$buffer.len()], $extra => [$type]); )*
    };

    // SLICE With tail with bytes_read
    ($buffer:expr, $index:expr, $bytes_read:expr, [$name:ident $(,$extra:ident)*]:$type:ident, $($tail:tt)*) => {
        // Get value from buffer into array
        $crate::deserialize_retriever!($bytes_read, $buffer[$index + $bytes_read..$buffer.len()], $name => [$type]);
        // Get value from buffer into array for extra
        $( $crate::deserialize_retriever!($bytes_read, $buffer[$index + $bytes_read..$buffer.len()], $extra => [$type]); )*
        // Parse tail
        $crate::deserialize_parser!($buffer, $index, $bytes_read, $($tail)*);
    };
}

/// Hidden extension of the to_buffer! macro. Retrieve value from buffer. Not meant to be used directly (although it will still work).
#[doc(hidden)]
#[macro_export]
macro_rules! deserialize_retriever {


    // Slice affectator
    ($bytes_read:expr, $buffer:expr, $name:ident => [$type:ident]) => {

        // Keep bytes size of u32
        let u32_bs = core::mem::size_of::<u32>();

        // Get size of slice
        let slice_size = <u32>::from_le_bytes($buffer[0..u32_bs].try_into().expect("Incorrect length!"));

        // Increase $bytes_read by u32 size
        $bytes_read += u32_bs;

        // Init vector
        let mut $name:Vec<$type> = Vec::new();

        // Retrieve each slice
        for i in 0..slice_size {

            // Use index 0 because $buffer[].try_into() consume buffer length
            $crate::deserialize_retriever!($bytes_read, $buffer, FB_TEMP_VARIABLE => $type);
            $name.push(FB_TEMP_VARIABLE);   // Push temporary variable into vector
        }       

    };


    /**********
    * BOOLEAN *
    **********/
    ($bytes_read:expr, $buffer:expr, $name:ident => bool) => {
        // Translate byte into u8
        let u8val = <u8>::from_le_bytes($buffer[0..core::mem::size_of::<u8>()].try_into().expect("Incorrect length!"));

        // Set bool value according to u8 value
        let $name = if u8val == 0 {
            false
        } else {
            true
        };

        // 1 byte was consumed for boolean
        $bytes_read += core::mem::size_of::<u8>();
    };


    /***********
    * NUMERICS * 
    ***********/
    ($bytes_read:expr, $buffer:expr, $name:ident => u8) => {
        let $name = <u8>::from_le_bytes($buffer[0..core::mem::size_of::<u8>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<u8>();
    };


    ($bytes_read:expr, $buffer:expr, $name:ident => u16) => {
        let $name = <u16>::from_le_bytes($buffer[0..core::mem::size_of::<u16>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<u16>();
    };


    ($bytes_read:expr, $buffer:expr, $name:ident => u32) => { 
        let $name = <u32>::from_le_bytes($buffer[0..core::mem::size_of::<u32>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<u32>();
    };


    ($bytes_read:expr, $buffer:expr, $name:ident => u64) => {
        let $name = <u64>::from_le_bytes($buffer[0..core::mem::size_of::<u64>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<u64>();
    };


    ($bytes_read:expr, $buffer:expr, $name:ident => u128) => {
        let $name = <u128>::from_le_bytes($buffer[0..core::mem::size_of::<u128>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<u128>();
    };


    ($bytes_read:expr, $buffer:expr, $name:ident => f32) => {
        let $name = <f32>::from_le_bytes($buffer[0..core::mem::size_of::<f32>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<f32>();
    };

    ($bytes_read:expr, $buffer:expr, $name:ident => f64) => {
        let $name = <f64>::from_le_bytes($buffer[0..core::mem::size_of::<f64>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<f64>();
    };


    ($bytes_read:expr, $buffer:expr, $name:ident => i8) => {
        let $name = <i8>::from_le_bytes($buffer[0..core::mem::size_of::<i8>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<i8>();
    };


    ($bytes_read:expr, $buffer:expr, $name:ident => i16) => {
        let $name = <i16>::from_le_bytes($buffer[0..core::mem::size_of::<i16>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<i16>();
    };


    ($bytes_read:expr, $buffer:expr, $name:ident => i32) => {
        let $name = <i32>::from_le_bytes($buffer[0..core::mem::size_of::<i32>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<i32>();
    };


    ($bytes_read:expr, $buffer:expr, $name:ident => i64) => {
        let $name = <i64>::from_le_bytes($buffer[0..core::mem::size_of::<i64>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<i64>();
    };


    ($bytes_read:expr, $buffer:expr, $name:ident => i128) => {
        let $name = <i128>::from_le_bytes($buffer[0..core::mem::size_of::<i128>()].try_into().expect("Incorrect length!"));
        $bytes_read += core::mem::size_of::<i128>();
    };
    /*********
    * STRING * 
    *********/
    ($bytes_read:expr, $buffer:expr, $name:ident => String) => {

        // Keep bytes size of u32
        let u32_bs = core::mem::size_of::<u32>();
        
        // Get size of string to retrieve
        let string_size = (<u32>::from_le_bytes($buffer[0..u32_bs].try_into().expect("Incorrect length!"))) as usize;

        // Use String::from_utf8 which is SAFE https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8
        let $name = String::from_utf8($buffer[u32_bs..u32_bs + string_size].to_vec()).expect("UTF8 String incorrect!"); 

        // Return size used 
        $bytes_read += u32_bs + string_size;

    };

    /***************
    * TAMPON TRAIT * 
    ***************/
    ($bytes_read:expr, $buffer:expr, $name:ident => $tampon:ident) => {
        let temp = $tampon::deserialize(&$buffer);
        let $name = temp.0;
        $bytes_read += temp.1;
    };


}