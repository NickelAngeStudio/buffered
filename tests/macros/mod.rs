/*
* @file tests/macro/mod.rs
*
* @module tests::macro
*
* @brief Header of macros tests
* 
* @details
* Header of macros tests
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

// boolean_pack_size! macro tests
#[cfg(test)]
mod bytes_size_test;

// serialize!, deserialize! macros tests
#[cfg(test)]
mod de_serialize_test;

// buffer! integration macro tests
#[cfg(test)]
mod buffer_test;