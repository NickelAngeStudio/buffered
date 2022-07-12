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

/// ##### Variadic macro used to fill a buffer with multiple [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html). 
/// 
/// # Description
/// Variadic macro used to fill a buffer with multiple [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html). 
/// 
/// Also work with [`slice`] of those types by using brackets `[]` instead of parenthesis `()` after the token type.
/// 
/// # Usage
/// `to_buffer!(buffer, index, [0..n]token(v1, ..., vn), [0..n]token[s1, ..., sn])`
/// * `buffer` - Mutable reference to [`slice`] of [`u8`] to copy bytes into.
/// * `index` - Index of the buffer to start copy into (use 0 to start at beginning).
/// * One-to-many `token(v1, ..., vn)` where elements in `parenthesis()` are the variables to be copied into buffer.
/// * One-to-many `token[s1, ..., sn]` where elements in `brackets[]` are the slices to be copied into buffer.
/// 
/// `token` type :
/// * `N` - Copy bytes of [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html).
/// * `S` - Copy bytes of [`String`].
/// * `T` - Copy bytes of implementors of [`Tampon`](trait.Tampon.html) trait.
/// 
/// # Return
/// Size in bytes of all arguments copied as [`usize`].
/// 
/// # Example(s)
/// ```
/// // Import macro bytes_size and to_buffer
/// use tampon::bytes_size;
/// use tampon::to_buffer;
/// 
/// // Declare multiple variables (numerics don't need to be same type)
/// let a:u8 = 55;
/// let b:u16 = 255;
/// let c:usize = 12545566;
/// let d:String = String::from("Example string");
/// let e:Vec<i32> = vec![i32::MAX; 50];
/// let f:Vec<f64> = vec![f64::MAX; 50];
/// 
/// // Get the size in bytes of all those elements in one macro call
/// let size = bytes_size!(N(a,b,c), S(d), N[e,f]);
/// 
/// // Create a mutable buffer long enough to store variables
/// let mut buffer:Vec<u8> = vec![0;size];
/// 
/// // Copy variables content into buffer
/// let copy_size = to_buffer!(buffer, 0, N(a,b,c), S(d), N[e,f]);
/// 
/// // Print result
/// println!("Bytes size of variables a,b,c,d,e,f is {}\nCopied size is {}\nResulted buffer={:?}", size, copy_size, buffer);
/// ```
/// ##### Buffer smaller than content will cause a panic! :
/// ``` should_panic
/// // Import macro bytes_size and to_buffer
/// use tampon::bytes_size;
/// use tampon::to_buffer;
/// 
/// // Declare multiple variables (numerics don't need to be same type)
/// let a:u8 = 55;
/// let b:u16 = 255;
/// let c:usize = 12545566;
/// let d:String = String::from("Example string");
/// let e:Vec<i32> = vec![i32::MAX; 50];
/// let f:Vec<f64> = vec![f64::MAX; 50];
/// 
/// // Mutable buffer TOO SMALL to contains variable
/// let mut buffer:Vec<u8> = vec![0;50];
/// 
/// // Copy variables content into buffer (will panic!)
/// let copy_size = to_buffer!(buffer, 0, N(a,b,c), S(d), N[e,f]);
/// ```
/// 
/// # Note(s)
/// * Works only with [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// * [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html) bytes are written as [`little endian`](https://en.wikipedia.org/wiki/Endianness).
/// 
/// # Panic(s)
/// * Will panic! if `buffer` length is smaller than all sources length combined.
#[macro_export]
macro_rules! to_buffer {
    // Macro built with Incremental TT munchers pattern : https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html

    
    // Return 0 on empty
    () => {{ 0 } as usize };

    ($buffer:expr, $index:expr) => {{ 0 } as usize };

    /****************
    * NUMERIC TYPES *
    ****************/
    // Without tail and separator
    ($buffer:expr, $index:expr, N($expr:expr $(,$extra:expr)*)) => {{

        // Bytes size used
        let mut bytes_size:usize = 0;

        // Get expression in slice of little endian bytes
        let src = $expr.to_le_bytes();

        // Copy to buffer via copy from slice
        $buffer[$index..($index + src.len())].copy_from_slice(&src);

        // Increment bytes_size
        bytes_size += src.len();
       
        // Loop extra arguments of N()
        $( bytes_size += to_buffer!($buffer, $index + bytes_size, N($extra)); )*

        // Return bytes_size
        bytes_size

    } as usize };

    // With tail and separator
    ($buffer:expr, $index:expr, N($expr:expr $(,$extra:expr)*), $($tail:tt)*) => {{

        // Bytes size used
        let mut bytes_size:usize = 0;

        // Get expression in slice of little endian bytes
        let src = $expr.to_le_bytes();

        // Copy to buffer via copy from slice
        $buffer[$index..($index + src.len())].copy_from_slice(&src);

        // Increment bytes_size
        bytes_size += src.len();

        // Loop extra arguments of N()
        $( bytes_size += to_buffer!($buffer, $index + bytes_size, N($extra)); )*

        // Expand tail arguments
        bytes_size += to_buffer!($buffer, $index + bytes_size, $($tail)*);

        // Return bytes_size
        bytes_size

    } as usize };


    /*********
    * STRING * 
    *********/
    // Without tail and separator
    ($buffer:expr, $index:expr, S($expr:expr $(,$extra:expr)*)) => {{

        // Bytes size used
        let mut bytes_size:usize = 0;

        // Get expression in slice of little endian bytes
        let src = $expr.as_bytes();

        // Copy to buffer via copy from slice
        $buffer[$index..($index + src.len())].copy_from_slice(&src);

        // Increment bytes_size
        bytes_size += src.len();
       
        // Loop extra arguments of S()
        $( bytes_size += to_buffer!($buffer, $index + bytes_size, S($extra)); )*

        // Return bytes_size
        bytes_size

    } as usize };

    // With tail and separator
    ($buffer:expr, $index:expr, S($expr:expr $(,$extra:expr)*), $($tail:tt)*) => {{

        // Bytes size used
        let mut bytes_size:usize = 0;

        // Get expression in slice of little endian bytes
        let src = $expr.as_bytes();

        // Copy to buffer via copy from slice
        $buffer[$index..($index + src.len())].copy_from_slice(&src);

        // Increment bytes_size
        bytes_size += src.len();

        // Loop extra arguments of S()
        $( bytes_size += to_buffer!($buffer, $index + bytes_size, S($extra)); )*

        // Expand tail arguments
        bytes_size += to_buffer!($buffer, $index + bytes_size, $($tail)*);

        // Return bytes_size
        bytes_size

    } as usize };

    /***************
    * TAMPON TRAIT *
    ***************/
    // Without tail and separator
    ($buffer:expr, $index:expr, T($expr:expr $(,$extra:expr)*)) => {{

        // Copy bytes and get bytes size used
        //let mut bytes_size:usize = tampon::Tampon::to_buffer(&$expr, &mut $buffer[$index..$buffer.len()]);
        let buffer_len = $buffer.len();
        let mut bytes_size:usize = $expr.to_buffer(&mut $buffer[$index..buffer_len]);
       
        // Loop extra arguments of T()
        $( bytes_size += to_buffer!($buffer, $index + bytes_size, T($extra)); )*

        // Return bytes_size
        bytes_size

    } as usize };

    // With tail and separator
    ($buffer:expr, $index:expr, T($expr:expr $(,$extra:expr)*), $($tail:tt)*) => {{

        // Copy bytes and get bytes size used
        //let mut bytes_size:usize = tampon::Tampon::to_buffer(&$expr, &mut $buffer[$index..$buffer.len()]);
        let buffer_len = $buffer.len();
        let mut bytes_size:usize = $expr.to_buffer(&mut $buffer[$index..buffer_len]);

        // Loop extra arguments of S()
        $( bytes_size += to_buffer!($buffer, $index + bytes_size, T($extra)); )*

        // Expand tail arguments
        bytes_size += to_buffer!($buffer, $index + bytes_size, $($tail)*);

        // Return bytes_size
        bytes_size

    } as usize };

     /********
    * SLICES *
    *********/
    // Slice without tail and separator
    ($buffer:expr, $index:expr, $token:ident[$expr:expr $(,$extra:expr)*]) => {{

        let mut bytes_size = 0;

        // Loop elements in first slice expression
        for elem in $expr.iter() {
            bytes_size += to_buffer!($buffer, $index + bytes_size, $token(elem));
        }

        // Loop extra arguments of $token[]
        $( bytes_size += to_buffer!($buffer, $index + bytes_size, $token[$extra]); )*
        
        // Return bytes_size
        bytes_size


    } as usize };
    // Slice with tail and separator
    ($buffer:expr, $index:expr, $token:ident[$expr:expr $(,$extra:expr)*], $($tail:tt)*) => {{

        let mut bytes_size = 0;

        // Loop elements in first slice expression
        for elem in $expr.iter() {
            bytes_size += to_buffer!($buffer, $index + bytes_size, $token(elem));
        }

        // Loop extra arguments of N[]
        $( bytes_size += to_buffer!($buffer, $index + bytes_size, $token[$extra]); )*

        // Expand tail arguments
        bytes_size += to_buffer!($buffer, $index + bytes_size, $($tail)*);
        
        // Return bytes_size
        bytes_size

    } as usize };
}