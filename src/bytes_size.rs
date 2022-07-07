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
/// Also work with [`slice`] of those types by using brackets `[]` instead of parenthesis `()` after the token type.
///
/// # Argument(s)
/// * One-to-many `token(v1, ..., vn)` where elements in `parenthesis()` are the variables to be sized.
/// * One-to-many `token[s1, ..., sn]` where elements in `brackets[]` are the slices to be sized.
/// 
/// `token` type :
/// * `N` - Count bytes of [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html).
/// * `S` - Count bytes of [`String`].
/// * `T` - Count bytes of implementors of [`Tampon`](trait.Tampon.html) trait.
/// 
/// # Return
/// Size in bytes of all arguments as [`usize`].
/// 
/// # Example(s)
/// ```
/// // Import macro
/// use tampon::bytes_size;
/// 
/// // Declare multiple variables (numerics don't need to be same type)
/// let a:u8 = 55;
/// let b:u16 = 255;
/// let c:usize = 12545566;
/// let d:String = String::from("Example string");
/// let e:Vec<i32> = vec![i32::MAX; 50];
/// let f:Vec<f64> = vec![f64::MAX; 50];
/// 
/// // Get the size of all those elements in one macro call
/// let size = bytes_size!(N(a,b,c), S(d), N[e,f]);
/// 
/// // Print result
/// println!("Bytes size of variables a,b,c,d,e,f is {}", size);
/// ```
/// 
/// # Note(s)
/// * Works only with [`Numeric types`](https://doc.rust-lang.org/reference/types/numeric.html), [`String`] and implementors of trait [`Tampon`](trait.Tampon.html).
/// 
#[macro_export]
macro_rules! bytes_size {
    // Macro built with Incremental TT munchers pattern : https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html

    // Return 0 on empty
    () => {{ 0 } as usize };

    /****************
    * NUMERIC TYPES *
    ****************/
    // Without comma separator
    (N($expr:expr $(,$extra:expr)*)) => {{
        core::mem::size_of_val(&$expr) + bytes_size!( $(N($extra),)* )
    } as usize };
    // With comma separator
    (N($expr:expr $(,$extra:expr)*), $($tail:tt)*) => {{
        core::mem::size_of_val(&$expr) + bytes_size!( $(N($extra),)* ) + bytes_size!( $($tail)* )
    } as usize };

    // Slice without comma separator
    (N[$expr:expr $(,$extra:expr)*]) => {{
        if $expr.len() > 0 {
            core::mem::size_of_val(&$expr[0]) * $expr.len() + bytes_size!( $(N[$extra],)* )
        } else {
            bytes_size!( $(N[$extra],)* )
        }
    } as usize };
    // Slice with comma separator
    (N[$expr:expr $(,$extra:expr)*], $($tail:tt)*) => {{
        if $expr.len() > 0 {
            core::mem::size_of_val(&$expr[0]) * $expr.len() + bytes_size!( $(N[$extra],)* ) + bytes_size!( $($tail)* )
        } else {
            bytes_size!( $(N[$extra],)* ) + bytes_size!( $($tail)* )
        }
    } as usize };

    /*********
    * STRING * 
    *********/
    // Without comma separator
    (S($expr:expr $(,$extra:expr)*)) => {{
        // String::len() gives size of string in bytes (https://doc.rust-lang.org/std/string/struct.String.html#method.len-1)
        $expr.len() + bytes_size!( $(S($extra),)* )
    } as usize };
    // With comma separator
    (S($expr:expr $(,$extra:expr)*), $($tail:tt)*) => {{
        // String::len() gives size of string in bytes (https://doc.rust-lang.org/std/string/struct.String.html#method.len-1)
        $expr.len() + bytes_size!( $(S($extra),)* ) + bytes_size!( $($tail)* )
    } as usize };

    // Slice without comma separator
    (S[$expr:expr $(,$extra:expr)*]) => {{
        let mut bytes_size = 0;
        for elem in $expr.iter() {
            bytes_size += elem.len();
        }
        bytes_size + bytes_size!( $(S[$extra],)* )
    } as usize };
    // Slice with comma separator
    (S[$expr:expr $(,$extra:expr)*], $($tail:tt)*) => {{
        let mut bytes_size = 0;
        for elem in $expr.iter() {
            bytes_size += elem.len()
        }
        bytes_size + bytes_size!( $(S[$extra],)* ) + bytes_size!( $($tail)* )
    } as usize };

    /***************
    * TAMPON TRAIT *
    ***************/
    // Without comma separator
    (T($expr:expr $(,$extra:expr)*)) => {{
        $expr.bytes_size() + bytes_size!( $(T($extra),)* )
    } as usize };
    // With comma separator
    (T($expr:expr $(,$extra:expr)*), $($tail:tt)*) => {{
        $expr.bytes_size() + bytes_size!( $(T($extra),)* ) + bytes_size!( $($tail)* )
    } as usize };

    // Slice without comma separator
    (T[$expr:expr $(,$extra:expr)*]) => {{
        let mut bytes_size = 0;
        for elem in $expr.iter() {
            bytes_size += elem.bytes_size();
        }
        bytes_size + bytes_size!( $(T[$extra],)* )
    } as usize };
    // Slice with comma separator
    (T[$expr:expr $(,$extra:expr)*], $($tail:tt)*) => {{
        let mut bytes_size = 0;
        for elem in $expr.iter() {
            bytes_size += elem.bytes_size();
        }
        bytes_size + bytes_size!( $(T[$extra],)* ) + bytes_size!( $($tail)* )
    } as usize };

}