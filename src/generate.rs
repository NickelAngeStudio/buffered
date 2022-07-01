/*
 * @file tampon/generate.rs
 *
 * @module tampon
 *
 * @brief Contain function and parameters to generate a buffer.
 * 
 * @details
 * Contain function and parameters to generate a buffer.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-07-01 - Happy Canada Day
 *
 * @version
 * 1.0 : 2022-07-01 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

/// ##### Generate a random [`u8`] buffer with specific size and [`charset`](buffer_generator_charset/index.html).
/// 
/// The generated buffer can be used as password, as a crypto mask, etc...
/// 
/// You have to supply your own random number generator that implement the [`rand::Rng`] trait. ([`rand`] crates supplies
/// a lot of those.)
/// # Argument(s)
/// * `rng` - Mutable [`rand::Rng`] reference used to generate the buffer.
/// * `size` - Size of the buffer generated.
/// * `charset` - [`buffer_generator_charset`] character set flags.
/// 
/// # Example(s)
/// ```
/// // Use rand crate
/// use rand::{Rng, prelude::ThreadRng};
/// 
/// // Use Tampon crate
/// use tampon::{self, buffer_generator_charset };
/// 
/// // Create number generator
/// let mut rng = rand::thread_rng();
///
/// // Generate buffer containing numbers and lower case of size 50
/// let buffer = tampon::generate_buffer(&mut rng, 50, 
///     buffer_generator_charset::LOWER_CASE | buffer_generator_charset::NUMBER);
///
/// // Print generated buffer
/// println!("Buffer = [{}]", String::from_utf8_lossy(&buffer));
/// 
/// ```
/// 
/// # Panic(s)
/// * generate_buffer() will panic if no `charset` buffer_generator_flag matches.
/// * generate_buffer() will panic if `size` == 0.
pub fn generate_buffer(rng : &mut impl rand::Rng, size : usize, charset: u8) -> Vec<u8> {

    // Make sure size generated > 0
    assert!(size > 0);

    // Make sure we use valid charset.
    assert!(buffer_generator_charset::validate_charset(charset));

    // Buffer to be filled
    let mut buffer: Vec<u8> = vec![0; size];

    // Vector of charset range to use
    let charset_range = buffer_generator_range::generate_charset_range(charset);

    // Fill buffer with character sets
    for i in 0..size {
        // Fill buffer with character from sample range
        buffer[i] = charset_range[rng.gen_range(0..charset_range.len())];
    }
    
    // Return generated buffer
    buffer
}


/// ##### Buffer generator flags used to provide character sets when using [`generate_buffer`].
/// Uses [`Rust bitwise operators`](https://www.tutorialspoint.com/rust/rust_bitwise_operators.htm) `| (BitWise OR)`
/// and `& (Bitwise AND)` to generate a charset range with available [`constants`](#constants) flags.
/// # Diagram(s)
/// ##### Ascii table showing u8 characters.
/// // TODO: Add ascii table link
/// # Example(s)
/// ```
/// // Import `buffer_generator_charset`
/// use tampon::buffer_generator_charset;
/// 
/// // Create a charset using bitwise `|` that will use numbers, lower case and symbols
/// let charset : u8 = buffer_generator_charset::NUMBER | buffer_generator_charset::LOWER_CASE | buffer_generator_charset::SYMBOL;
/// 
/// // Validate that charset contains flag by using bitwise `&`.
/// if charset & buffer_generator_charset::NUMBER > 0 {
///     println!("Charset contains NUMBER!");
/// }
/// if charset & buffer_generator_charset::UPPER_CASE == 0 {
///     println!("Charset doesn't containts UPPER_CASE!");
/// }
/// ```
pub mod buffer_generator_charset {
    /// Include number 0..9 (10 characters) when generating buffer.
    pub const NUMBER: u8 = 1;

    /// Include lower case a..z (26 characters) when generating buffer.
    pub const LOWER_CASE: u8 = 2;

    /// Include upper case A..Z (26 characters) when generating buffer.
    pub const UPPER_CASE: u8 = 4;

    /// Include symbols (!,#,$,%,...) (32 characters) when generating buffer.
    pub const SYMBOL: u8 = 8;

    /// Include unreadable symbols (NULL, TAB, BS, DEL ,...) (162 characters) when generating buffer.
    /// 
    /// Printing the buffer will show unreadable characters.
    pub const UNREADABLE: u8 = 16;

    /// Include all characters including unreadable (256 characters).
    pub const ALL: u8 = 31;

    /// ##### Function that validate if argument charset is valid.
    /// # Argument(s)
    /// * `charset` - Charset flag containing charset options.
    /// # Return
    /// True if contains at least 1 valid charset flag, false otherwise.
    #[doc(hidden)]
    pub fn validate_charset(charset: u8) -> bool {

        ((charset & NUMBER) + (charset & LOWER_CASE) +
        (charset & UPPER_CASE) + (charset & SYMBOL) +
        (charset & UNREADABLE)) > 0

    }
}


/// Definition of character set ranges constants
#[doc(hidden)]
pub(crate) mod buffer_generator_range {
    pub const NUMBER_RANGE_0 : std::ops::RangeInclusive<u8> = 48..=57;

    pub const LOWER_CASE_RANGE_0 : std::ops::RangeInclusive<u8> = 97..=122;

    pub const UPPER_CASE_RANGE_0 : std::ops::RangeInclusive<u8> = 65..=90;

    pub const SYMBOL_RANGE_0 : std::ops::RangeInclusive<u8> = 33..=47;
    pub const SYMBOL_RANGE_1 : std::ops::RangeInclusive<u8> = 58..=64;
    pub const SYMBOL_RANGE_2 : std::ops::RangeInclusive<u8> = 91..=96;
    pub const SYMBOL_RANGE_3 : std::ops::RangeInclusive<u8> = 123..=126;

    pub const UNREADABLE_RANGE_0 : std::ops::RangeInclusive<u8> = 0..=32;
    pub const UNREADABLE_RANGE_1 : std::ops::RangeInclusive<u8> = 127..=255;

    pub const ALL_RANGE_0 : std::ops::RangeInclusive<u8> = 0..=255;



    /// Generate a Vec<u8> from the charset as a valid charset sample.
    pub(crate) fn generate_charset_range(charset: u8) -> Vec<u8>{

        // Create 
        let mut charset_range : Vec<u8> = Vec::new();

        // If all characters set are used
        if charset == super::buffer_generator_charset::ALL {
            for i in ALL_RANGE_0 {
                charset_range.push(i);
            }
        } else {
            // Add number to charset
            if charset & super::buffer_generator_charset::NUMBER > 0 {
                for i in NUMBER_RANGE_0 {
                    charset_range.push(i);
                }
            }

            // Add lower case to charset
            if charset & super::buffer_generator_charset::LOWER_CASE > 0 {
                for i in LOWER_CASE_RANGE_0 {
                    charset_range.push(i);
                }
            }

            // Add upper case to charset
            if charset & super::buffer_generator_charset::UPPER_CASE > 0 {
                for i in UPPER_CASE_RANGE_0 {
                    charset_range.push(i);
                }
            }

            // Add symbol to charset
            if charset & super::buffer_generator_charset::SYMBOL > 0 {
                for i in SYMBOL_RANGE_0 {
                    charset_range.push(i);
                }
                for i in SYMBOL_RANGE_1 {
                    charset_range.push(i);
                }
                for i in SYMBOL_RANGE_2 {
                    charset_range.push(i);
                }
                for i in SYMBOL_RANGE_3 {
                    charset_range.push(i);
                }
            }

            // Add unreadable to charset
            if charset & super::buffer_generator_charset::UNREADABLE > 0 {
                for i in UNREADABLE_RANGE_0 {
                    charset_range.push(i);
                }
                for i in UNREADABLE_RANGE_1 {
                    charset_range.push(i);
                }
            } 
    }

        // Return charset_range
        charset_range
    }
}