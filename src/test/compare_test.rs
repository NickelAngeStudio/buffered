/*
 * @file tampon/test/compare_test.rs
 *
 * @module tampon::test
 *
 * @brief Contains tests for compare_buffers function.
 * 
 * @details
 * Contains tests for compare_buffers function.
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

 // Values into buffers
 static BUFFER_VALUE_MAX: u8 = 100;
 static BUFFER_VALUE_MIN: u8 = 5;

 // Size of buffer for tests
static BUFFER_SIZE_MAX: usize = 255;
static BUFFER_SIZE_MIN: usize = 100;

#[test]
// Compare identical buffers
fn compare_identical() {
    let b1:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];
    let b2:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];

    let diff = crate::compare_buffers(&b1,&b2);
    println!("Diff={}",diff);
    // Both buffer should be equal and return 0
    assert!(diff==0);    
}

#[test]
// Compare identical buffers with no values
fn compare_identical_no_size() {
    let b1:Vec<u8> = Vec::new();
    let b2:Vec<u8> = Vec::new();
    
    let diff = crate::compare_buffers(&b1,&b2);
    println!("Diff={}",diff);
    // Both buffer should be equal and return 0
    assert!(diff==0);    
}

#[test]
// Compare buffers with different values but same size
fn compare_different_value() {
    let b1:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];
    let b2:Vec<u8> = vec![BUFFER_VALUE_MIN;BUFFER_SIZE_MAX];
    
    // Both buffer should be bigger than 0.
    assert!(crate::compare_buffers(&b1,&b2)>0);    
}

#[test]
// Compare buffers with same value but different sizes
fn compare_different_size() {
    let b1:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];
    let b2:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MIN];
    
    let diff = crate::compare_buffers(&b1,&b2);
    println!("Diff={}",diff);
   // Both buffer should be bigger than 0.
    assert!(diff==(BUFFER_SIZE_MAX - BUFFER_SIZE_MIN));    
}

#[test]
// Compare buffer vs an empty buffer
fn compare_different_size_no_size() {
    let b1:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];
    let b2:Vec<u8> = Vec::new();
    
    let diff = crate::compare_buffers(&b1,&b2);
    println!("Diff={}",diff);
    // Both buffer should be bigger than 0.
    assert!(diff==(BUFFER_SIZE_MAX - 0));    
}

// Compare a buffer that has different size and values
#[test]
fn compare_different_value_and_size() {
    let b1:Vec<u8> = vec![BUFFER_VALUE_MAX;BUFFER_SIZE_MAX];
    let b2:Vec<u8> = vec![BUFFER_VALUE_MIN;BUFFER_SIZE_MIN];
    
    let diff = crate::compare_buffers(&b1,&b2);
    println!("Diff={}",diff);
    // Both buffer should be bigger than 0.
    assert!(diff>0);    
}