/*
 * @file tampon/test/wipe_test.rs
 *
 * @module tampon::test
 *
 * @brief Contains tests for wipe_buffer function.
 * 
 * @details
 * Contains tests for wipe_buffer function.
 *
 * @author Mathieu Grenier
 * @copyright NickelAnge.Studio
 *
 * @date 2022-07-01
 *
 * @version
 * 1.0 : 2022-07-01 | Mathieu Grenier | Code creation
 *
 * @ref
 * 
 * @todo
 */

use crate::buffer_generator_charset;

// Size of buffer for tests
static BUFFER_SIZE: usize = 255;

#[test]
fn wipe_generated_buffer() {

    let mut rng = rand::thread_rng();
    let charset = buffer_generator_charset::NUMBER | buffer_generator_charset::LOWER_CASE | 
                buffer_generator_charset::UPPER_CASE | buffer_generator_charset::SYMBOL;

    let mut buffer = crate::generate_buffer(&mut rng, BUFFER_SIZE, charset);

    println!("Buffer before wipe = {:?}", buffer);

    crate::wipe_buffer(&mut buffer);

    // TODO: Add compare

    println!("Buffer after wipe = {:?}", buffer);
}