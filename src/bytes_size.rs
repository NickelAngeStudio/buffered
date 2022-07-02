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

/// ##### Variadic macro used to get the size of bytes of multiple primitives, vectors and implementors of trait [`Tampon`](trait.Tampon.html). 
/// 
/// Variadic macro used to get the size of bytes of multiple primitives, vectors and implementors of trait [`Tampon`](trait.Tampon.html). 
/// 
/// # Example(s)
/// ```
/// //TODO: Add example
/// ```
/// 
/// # Note(s)
/// * Works only with [`primitive`](https://doc.rust-lang.org/rust-by-example/primitives.html), [`slice`] of primitives
/// and implementors of trait [`Tampon`](trait.Tampon.html).
/// 
/// # Argument(s)
/// * `<token>(var,type)` - 1..n variables to get size of according to `<token>` types :
///     * `p` - Count bytes of primitive.
///     * `ps` - Count bytes of [`slice`] of primitives.
///     * `t` - Count bytes of implementors of [`Tampon`](trait.Tampon.html) trait.
///     * `ts` - Count bytes of [`slice`] of implementors of [`Tampon`](trait.Tampon.html) trait.
#[macro_export]
macro_rules! bytes_size {

    // Non-variadic with only primitive
    (p ($primitive:expr, $type:ty)) => {
        // Return size of primitive
        //core::mem::size_of::<$type>()
        core::mem::size_of::<$type>()
    };

    // Non-variadic with slice of primitives
    (ps ($primitives:expr, $type:ty)) => {
        // Return size of primitive
        core::mem::size_of::<$type>() * $primitives.len()
    };

    // Non-variadic with trait Tampon implementator
    (t ($tampon:expr, $type:ty)) => {
        // Return size of trait
        $tampon.bytes_size()
    };

    // Non-variadic with slice of Tampon implementator
    (ts ($tampons:expr, $type:ty)) => {

        if $tampons.len() > 0 {
            $tampons[0].bytes_size() * $tampons.len()
        } 
        else {
            // Return 0 if no elements in traits slice
            0
        }
    };

    // Variadic with primitive
    (p ($primitive:expr, $type:ty), $($token:tt ($ext_expr:expr, $ext_type:ty)), *) => {
        // Return size of primitive
        core::mem::size_of::<$type>() + bytes_size!($($token($ext_expr, $ext_type)), *)
    };

    // Variadic with slice of primitives
    (ps ($primitives:expr, $type:ty), $($token:tt ($ext_expr:expr, $ext_type:ty)), *) => {
        // Return size of primitive
        core::mem::size_of::<$type>() * $primitives.len() + bytes_size!($($token($ext_expr, $ext_type)), *)
    };

    // Variadic with trait Tampon implementator
    (t ($tampon:expr, $type:ty), $($token:tt ($ext_expr:expr, $ext_type:ty)), *) => {
        // Return size of trait
        $tampon.bytes_size() + bytes_size!($($token($ext_expr, $ext_type)), *)
    };

    // Variadic with slice of Tampon implementator
    (ts ($tampons:expr, $type:ty), $($token:tt ($ext_expr:expr, $ext_type:ty)), *) => {

        if $tampons.len() > 0 {
            $tampons[0].bytes_size() * $tampons.len() + bytes_size!($($token($ext_expr, $ext_type)), *)
        } 
        else {
            // Return 0 if no elements in traits slice
            0 + bytes_size!($($token($ext_expr, $ext_type)), *)
        }
    };




}


#[cfg(test)]
mod test{
    #[test]
    fn test_mac() {
        let a:u8 = 35;
        let b = 655535;
        let size = bytes_size!( 
            p(a, u8),
            p(b, i32)
        );

        println!("Size={}", size);
    }
}