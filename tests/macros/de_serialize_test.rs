/*
 * @file tampon/tests/serialize_test.rs
 *
 * @module tampon::tests
 *
 * @brief Contains unit and integration tests for serialize! and deserialize! macro.
 * 
 * @details
 * Contains unit and integration tests for serialize! and deserialize! macro.
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
 */


use tampon::{serialize, deserialize};
pub use tampon::Tampon;
use crate::implementation::{ TamponS1, TamponS2 };

use crate::tampon_slice;
use crate::{data::{ do_vecs_match, STRINGS, do_vecs_eq_match}, boolean_var, boolean_slice, numeric_var, numeric_slice, string_var, string_slice, tampon_var};

#[test]
#[should_panic]
// Test serialize without enough space
fn serialize_too_small(){
    let mut _var_size = 0;
    numeric_var!(_var_size, to_n0:u8, to_n1:u16, to_n2:u32, to_n3:u64, to_n4:u128, to_n5:f32, to_n6:f64,
        to_n7:i8, to_n8:i16, to_n9:i32, to_n10:i64, to_n11:i128);
    let mut buffer:Vec<u8> = vec![0;10];

    serialize!(buffer, (to_n0):u8, (to_n1):u16, (to_n2):u32, (to_n3):u64, (to_n4):u128, (to_n5):f32, (to_n6):f64,
    (to_n7):i8, (to_n8):i16, (to_n9):i32, (to_n10):i64, (to_n11):i128);
}

#[test]
#[should_panic]
// Test deserialize with buffer smaller than targets
fn deserialize_too_small(){
    // Any buffer made too small for data to retrieve.
    let buffer: Vec<u8> = vec![202, 17, 145, 184, 199, 210, 231, 3, 240, 64, 30, 110, 191, 0, 174, 248, 6, 0, 14, 0, 0, 0, 69, 120, 97, 109, 112, 108, 101, 32, 115, 116, 114, 105, 110];
    // Deserialize data from buffer (will panic!)
    deserialize!(buffer, bytes_read, (_a):u16, (_b):f64, (_c,_d):u32, (_e):String, [_f]:i32);
}

#[test]
// Test to > from buffer of 1 bool
fn serialize_deserialize_1_bool(){
    // Create variables and get their size
    let mut var_size = 0;
    boolean_var!(var_size, to_b0);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_b0):bool);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_b0):bool);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_b0 == to_b0);
    println!("Value retrieved successfully!");
}


#[test]
// Test to > from buffer of 2 bool
fn serialize_deserialize_2_bools(){
    // Create variables and get their size
    let mut var_size = 0;
    boolean_var!(var_size, to_b0, to_b1);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_b0, to_b1):bool);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_b0, from_b1):bool);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_b0 == to_b0 && from_b1 == to_b1);
    println!("Value retrieved successfully!");
}


#[test]
// Test to > from buffer of 10 bool
fn serialize_deserialize_10_bools(){
    // Create variables and get their size
    let mut var_size = 0;
    boolean_var!(var_size, to_b0, to_b1, to_b2, to_b3, to_b4, to_b5, to_b6, to_b7, to_b8, to_b9);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_b0,to_b1):bool, (to_b2):bool, (to_b3,to_b4,to_b5,to_b6,to_b7):bool, (to_b8,to_b9):bool);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_b0,from_b1):bool, (from_b2):bool, (from_b3,from_b4,from_b5,
        from_b6,from_b7):bool, (from_b8,from_b9):bool);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_b0 == to_b0 && from_b1 == to_b1 && from_b2 == to_b2 && from_b3 == to_b3 && from_b4 == to_b4
        && from_b5 == to_b5 && from_b6 == to_b6 && from_b7 == to_b7 && from_b8 == to_b8 && from_b9 == to_b9);
    println!("Value retrieved successfully!");
}


#[test]
// Test to > from buffer of 1 bool slice
fn serialize_deserialize_1_bool_slice(){
    // Create variables and get their size
    let mut var_size = 0;
    boolean_slice!(var_size, 0, to_bs0);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_bs0]:bool);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_bs0]:bool);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_match(&from_bs0, &to_bs0));
    println!("Value retrieved successfully!");

    
}

#[test]
// Test to > from buffer of 2 bool slice
fn serialize_deserialize_2_bool_slices(){
    // Create variables and get their size
    let mut var_size = 0;
    boolean_slice!(var_size, 0, to_bs0, to_bs1);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_bs0, to_bs1]:bool);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_bs0, from_bs1]:bool);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_match(&from_bs0, &to_bs0) && do_vecs_match(&from_bs1, &to_bs1));
    println!("Value retrieved successfully!");

   
}


#[test]
// Test to > from buffer of 10 bool slices
fn serialize_deserialize_10_bool_slices(){
    // Create variables and get their size
    let mut var_size = 0;
    boolean_slice!(var_size, 0, to_bs0, to_bs1, to_bs2, to_bs3, to_bs4, to_bs5, to_bs6, to_bs7, to_bs8, to_bs9);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_bs0,to_bs1]:bool, [to_bs2]:bool, [to_bs3,to_bs4,to_bs5,to_bs6,to_bs7]:bool, 
        [to_bs8,to_bs9]:bool);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_bs0,from_bs1]:bool, [from_bs2]:bool, [from_bs3,from_bs4,from_bs5,
        from_bs6,from_bs7]:bool, [from_bs8,from_bs9]:bool);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_match(&from_bs0, &to_bs0) && do_vecs_match(&from_bs1, &to_bs1) && do_vecs_match(&from_bs2, &to_bs2)
    && do_vecs_match(&from_bs3, &to_bs3) && do_vecs_match(&from_bs4, &to_bs4) && do_vecs_match(&from_bs5, &to_bs5)
    && do_vecs_match(&from_bs6, &to_bs6) && do_vecs_match(&from_bs7, &to_bs7) && do_vecs_match(&from_bs8, &to_bs8)
    && do_vecs_match(&from_bs9, &to_bs9));
    println!("Value retrieved successfully!");
    
   
}

#[test]
// Test to > from buffer 1 numeric type
fn serialize_deserialize_1_numeric(){
    let mut var_size = 0;
    numeric_var!(var_size, to_n0:i8);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_n0):i8);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_n0):i8);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_n0 == to_n0);
    println!("Value retrieved successfully!");

}

#[test]
// Test to > from buffer 2 numeric type
fn serialize_deserialize_2_numerics(){
    let mut var_size = 0;
    numeric_var!(var_size, to_n0:f32, to_n1:i128);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_n0):f32, (to_n1):i128);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_n0):f32, (from_n1):i128);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_n0 == to_n0 && from_n1 == to_n1);
    println!("Value retrieved successfully!");
}

#[test]
// Test to > from buffer all numeric type
fn serialize_deserialize_all_numerics(){
    let mut var_size = 0;
    numeric_var!(var_size, to_n0:u8, to_n1:u16, to_n2:u32, to_n3:u64, to_n4:u128, to_n5:f32, to_n6:f64,
        to_n7:i8, to_n8:i16, to_n9:i32, to_n10:i64, to_n11:i128);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_n0):u8, (to_n1):u16, (to_n2):u32, (to_n3):u64, (to_n4):u128, (to_n5):f32, (to_n6):f64,
    (to_n7):i8, (to_n8):i16, (to_n9):i32, (to_n10):i64, (to_n11):i128);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_n0):u8, (from_n1):u16, (from_n2):u32, (from_n3):u64, (from_n4):u128, (from_n5):f32, (from_n6):f64,
    (from_n7):i8, (from_n8):i16, (from_n9):i32, (from_n10):i64, (from_n11):i128);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_n0 == to_n0 && from_n1 == to_n1 && from_n2 == to_n2 && from_n3 == to_n3
        && from_n4 == to_n4 && from_n5 == to_n5 && from_n6 == to_n6 && from_n7 == to_n7
        && from_n8 == to_n8 && from_n9 == to_n9 && from_n10 == to_n10 && from_n11 == to_n11);
    println!("Value retrieved successfully!");
}


#[test]
// Test to > from buffer 1 numeric slice
fn serialize_deserialize_1_numeric_slice(){
    let mut var_size = 0;
    numeric_slice!(var_size, 0, to_ns0:i8);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_ns0]:i8);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_ns0]:i8);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_match(&from_ns0, &to_ns0));
    println!("Value retrieved successfully!");

}

#[test]
// Test to > from buffer 2 numeric slices
fn serialize_deserialize_2_numeric_slices(){
    let mut var_size = 0;
    numeric_slice!(var_size, 0, to_ns0:f32, to_ns1:i128, to_ns2:f32);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_ns0, to_ns2]:f32, [to_ns1]:i128);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_ns0, from_ns2]:f32, [from_ns1]:i128);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_match(&from_ns0, &to_ns0) && do_vecs_match(&from_ns1, &to_ns1)  && do_vecs_match(&from_ns2, &to_ns2));
    println!("Value retrieved successfully!");

}

#[test]
// Test to > from buffer all numeric slices
fn serialize_deserialize_all_numeric_slices(){
    let mut var_size = 0;
    numeric_slice!(var_size, 0, to_ns0:u8, to_ns1:u16, to_ns2:u32, to_ns3:u64, to_ns4:u128, to_ns5:f32, to_ns6:f64,
        to_ns7:i8, to_ns8:i16, to_ns9:i32, to_ns10:i64, to_ns11:i128);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_ns0]:u8, [to_ns1]:u16, [to_ns2]:u32, [to_ns3]:u64, [to_ns4]:u128, [to_ns5]:f32, [to_ns6]:f64,
    [to_ns7]:i8, [to_ns8]:i16, [to_ns9]:i32, [to_ns10]:i64, [to_ns11]:i128);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_ns0]:u8, [from_ns1]:u16, [from_ns2]:u32, [from_ns3]:u64, [from_ns4]:u128, [from_ns5]:f32, [from_ns6]:f64,
        [from_ns7]:i8, [from_ns8]:i16, [from_ns9]:i32, [from_ns10]:i64, [from_ns11]:i128);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_match(&from_ns0, &to_ns0) && do_vecs_match(&from_ns1, &to_ns1) && do_vecs_match(&from_ns2, &to_ns2)
    && do_vecs_match(&from_ns3, &to_ns3) && do_vecs_match(&from_ns4, &to_ns4) && do_vecs_match(&from_ns5, &to_ns5)
    && do_vecs_match(&from_ns6, &to_ns6) && do_vecs_match(&from_ns7, &to_ns7) && do_vecs_match(&from_ns8, &to_ns8)
    && do_vecs_match(&from_ns9, &to_ns9) && do_vecs_match(&from_ns10, &to_ns10) && do_vecs_match(&from_ns11, &to_ns11));
    println!("Value retrieved successfully!");
}


#[test]
// Test to > from buffer 1 string
fn serialize_deserialize_1_string(){
    let mut var_size = 0;
    string_var!(var_size, STRINGS, 0, to_s0);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_s0):String);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_s0):String);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_s0.eq(&to_s0));
    println!("Value retrieved successfully!");
}


#[test]
// Test to > from buffer 2 strings
fn serialize_deserialize_2_strings(){
    let mut var_size = 0;
    string_var!(var_size, STRINGS, 0, to_s0, to_s1);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_s0, to_s1):String);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_s0, from_s1):String);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_s0.eq(&to_s0) && from_s1.eq(&to_s1));
    println!("Value retrieved successfully!");
}

#[test]
// Test to > from buffer 10 strings
fn serialize_deserialize_10_strings(){
    let mut var_size = 0;
    string_var!(var_size, STRINGS, 0, to_s0, to_s1, to_s2, to_s3, to_s4, to_s5, to_s6, to_s7, to_s8, to_s9);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_s0, to_s1):String, (to_s2, to_s3, to_s4):String, (to_s5):String, (to_s6, to_s7, to_s8, to_s9):String);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_s0, from_s1):String, (from_s2, from_s3, from_s4):String, (from_s5):String, 
    (from_s6, from_s7, from_s8, from_s9):String);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_s0.eq(&to_s0) && from_s1.eq(&to_s1) && from_s2.eq(&to_s2) && from_s3.eq(&to_s3) && from_s4.eq(&to_s4)
    && from_s5.eq(&to_s5) && from_s6.eq(&to_s6) && from_s7.eq(&to_s7) && from_s8.eq(&to_s8) && from_s9.eq(&to_s9));
    println!("Value retrieved successfully!");
}


#[test]
// Test to > from buffer 1 string slice
fn serialize_deserialize_1_string_slice(){
    let mut var_size = 0;
    string_slice!(var_size, STRINGS, 0, to_ss0);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_ss0]:String);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_ss0]:String);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_eq_match(&from_ss0, &to_ss0));
    println!("Value retrieved successfully!");
}

#[test]
// Test to > from buffer 2 string slices
fn serialize_deserialize_2_string_slices(){
    let mut var_size = 0;
    string_slice!(var_size, STRINGS, 0, to_ss0, to_ss1);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_ss0, to_ss1]:String);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_ss0, from_ss1]:String);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_eq_match(&from_ss0, &to_ss0) && do_vecs_eq_match(&from_ss1, &to_ss1));
    println!("Value retrieved successfully!");
}

#[test]
// Test to > from buffer 10 string slices
fn serialize_deserialize_10_string_slices(){
    let mut var_size = 0;
    string_slice!(var_size, STRINGS, 0, to_ss0, to_ss1, to_ss2, to_ss3, to_ss4, to_ss5, to_ss6, to_ss7, to_ss8, to_ss9);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_ss0, to_ss1]:String, [to_ss2, to_ss3, to_ss4]:String, [to_ss5]:String, 
        [to_ss6, to_ss7, to_ss8, to_ss9]:String);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_ss0, from_ss1]:String, [from_ss2, from_ss3, from_ss4]:String, [from_ss5]:String, 
        [from_ss6, from_ss7, from_ss8, from_ss9]:String);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_eq_match(&from_ss0, &to_ss0) && do_vecs_eq_match(&from_ss1, &to_ss1) && do_vecs_eq_match(&from_ss2, &to_ss2)
    && do_vecs_eq_match(&from_ss3, &to_ss3) && do_vecs_eq_match(&from_ss4, &to_ss4) && do_vecs_eq_match(&from_ss5, &to_ss5)
    && do_vecs_eq_match(&from_ss6, &to_ss6) && do_vecs_eq_match(&from_ss7, &to_ss7) && do_vecs_eq_match(&from_ss8, &to_ss8)
    && do_vecs_eq_match(&from_ss9, &to_ss9));
    println!("Value retrieved successfully!");
}

#[test]
// Test to > from buffer 1 implementor of Tampon trait
fn serialize_deserialize_1_tampon(){
    let mut var_size = 0;
    tampon_var!(var_size, to_t0:TamponS1);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_t0):TamponS1);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_t0):TamponS1);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_t0.eq(&to_t0));
    println!("Value retrieved successfully!");
}

#[test]
// Test to > from buffer 2 implementors of Tampon trait
fn serialize_deserialize_2_tampons(){
    let mut var_size = 0;
    tampon_var!(var_size, to_t0:TamponS1, to_t1:TamponS2);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_t0):TamponS1, (to_t1):TamponS2);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_t0):TamponS1, (from_t1):TamponS2);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_t0.eq(&to_t0) && from_t1.eq(&to_t1));
    println!("Value retrieved successfully!");
}

#[test]
// Test to > from buffer 10 implementors of Tampon trait
fn serialize_deserialize_10_tampons(){
    let mut var_size = 0;
    tampon_var!(var_size, to_t0:TamponS1, to_t1:TamponS2, to_t2:TamponS1, to_t3:TamponS1, to_t4:TamponS1,
        to_t5:TamponS2, to_t6:TamponS2, to_t7:TamponS1, to_t8:TamponS1, to_t9:TamponS2);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, (to_t0):TamponS1, (to_t1):TamponS2, (to_t2, to_t3, to_t4):TamponS1,
        (to_t5, to_t6):TamponS2, (to_t7, to_t8):TamponS1, (to_t9):TamponS2);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, (from_t0):TamponS1, (from_t1):TamponS2, (from_t2, from_t3, from_t4):TamponS1,
    (from_t5, from_t6):TamponS2, (from_t7, from_t8):TamponS1, (from_t9):TamponS2);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(from_t0.eq(&to_t0) && from_t1.eq(&to_t1) && from_t2.eq(&to_t2) && from_t3.eq(&to_t3) && from_t4.eq(&to_t4)
    && from_t5.eq(&to_t5) && from_t6.eq(&to_t6) && from_t7.eq(&to_t7) && from_t8.eq(&to_t8) && from_t9.eq(&to_t9));
    println!("Value retrieved successfully!");
}


#[test]
// Test to > from buffer 1 slice of implementor of Tampon trait
fn serialize_deserialize_1_tampon_slice(){
    let mut var_size = 0;
    tampon_slice!(var_size, 0, to_ts0:TamponS1);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_ts0]:TamponS1);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_ts0]:TamponS1);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_eq_match(&from_ts0, &to_ts0));
    println!("Value retrieved successfully!");
}

#[test]
// Test to > from buffer 2 slices of implementor of Tampon trait
fn serialize_deserialize_2_tampon_slices(){
    let mut var_size = 0;
    tampon_slice!(var_size, 0, to_ts0:TamponS1, to_ts1:TamponS1, to_ts2:TamponS2);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_ts0, to_ts1]:TamponS1, [to_ts2]:TamponS2);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_ts0, from_ts1]:TamponS1, [from_ts2]:TamponS2);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_eq_match(&from_ts0, &to_ts0) && do_vecs_eq_match(&from_ts1, &to_ts1) && do_vecs_eq_match(&from_ts2, &to_ts2));
    println!("Value retrieved successfully!");
}

#[test]
// Test to > from buffer 10 slices of implementor of Tampon trait
fn serialize_deserialize_10_tampon_slices(){
    let mut var_size = 0;
    tampon_slice!(var_size, 0, to_ts0:TamponS1, to_ts1:TamponS2, to_ts2:TamponS1, to_ts3:TamponS1, to_ts4:TamponS1,
        to_ts5:TamponS2, to_ts6:TamponS2, to_ts7:TamponS1, to_ts8:TamponS1, to_ts9:TamponS2);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, [to_ts0]:TamponS1, [to_ts1]:TamponS2, [to_ts2, to_ts3, to_ts4]:TamponS1, [to_ts5, to_ts6]:TamponS2,
        [to_ts7, to_ts8]:TamponS1, [to_ts9]:TamponS2);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, [from_ts0]:TamponS1, [from_ts1]:TamponS2, [from_ts2, from_ts3, from_ts4]:TamponS1, [from_ts5, from_ts6]:TamponS2,
        [from_ts7, from_ts8]:TamponS1, [from_ts9]:TamponS2);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(do_vecs_eq_match(&from_ts0, &to_ts0) && do_vecs_eq_match(&from_ts1, &to_ts1) && do_vecs_eq_match(&from_ts2, &to_ts2)
    && do_vecs_eq_match(&from_ts3, &to_ts3) && do_vecs_eq_match(&from_ts4, &to_ts4) && do_vecs_eq_match(&from_ts5, &to_ts5)
    && do_vecs_eq_match(&from_ts6, &to_ts6) && do_vecs_eq_match(&from_ts7, &to_ts7) && do_vecs_eq_match(&from_ts8, &to_ts8)
    && do_vecs_eq_match(&from_ts9, &to_ts9));
    println!("Value retrieved successfully!");
}

#[test]
// Test to > from buffer with everythings at the same time
fn serialize_deserialize_everythings(){
    let mut var_size = 0;
    boolean_var!(var_size, to_b0, to_b1, to_b2, to_b3, to_b4, to_b5, to_b6, to_b7, to_b8, to_b9);
    boolean_slice!(var_size, 0, to_bs0, to_bs1, to_bs2, to_bs3, to_bs4, to_bs5, to_bs6, to_bs7, to_bs8, to_bs9);
    numeric_var!(var_size, to_n0:u8, to_n1:u16, to_n2:u32, to_n3:u64, to_n4:u128, to_n5:f32, to_n6:f64,
        to_n7:i8, to_n8:i16, to_n9:i32, to_n10:i64, to_n11:i128);
    numeric_slice!(var_size, 0, to_ns0:u8, to_ns1:u16, to_ns2:u32, to_ns3:u64, to_ns4:u128, to_ns5:f32, to_ns6:f64,
        to_ns7:i8, to_ns8:i16, to_ns9:i32, to_ns10:i64, to_ns11:i128);
    string_var!(var_size, STRINGS, 0, to_s0, to_s1, to_s2, to_s3, to_s4, to_s5, to_s6, to_s7, to_s8, to_s9);
    string_slice!(var_size, STRINGS, 0, to_ss0, to_ss1, to_ss2, to_ss3, to_ss4, to_ss5, to_ss6, to_ss7, to_ss8, to_ss9);
    tampon_var!(var_size, to_t0:TamponS1, to_t1:TamponS2, to_t2:TamponS1, to_t3:TamponS1, to_t4:TamponS1,
        to_t5:TamponS2, to_t6:TamponS2, to_t7:TamponS1, to_t8:TamponS1, to_t9:TamponS2);
    tampon_slice!(var_size, 0, to_ts0:TamponS1, to_ts1:TamponS2, to_ts2:TamponS1, to_ts3:TamponS1, to_ts4:TamponS1,
        to_ts5:TamponS2, to_ts6:TamponS2, to_ts7:TamponS1, to_ts8:TamponS1, to_ts9:TamponS2);

    // Create buffer with serialize!
    let mut buffer:Vec<u8> = vec![0;var_size];
    serialize!(buffer, to_size, 
        [to_ts2, to_ts3, to_ts4]:TamponS1, [to_ts5, to_ts6]:TamponS2,
        (to_b0,to_b1):bool, (to_b2):bool, (to_b3,to_b4,to_b5,to_b6,to_b7):bool,
        (to_s0, to_s1):String, (to_s2, to_s3, to_s4):String,
        [to_bs0,to_bs1]:bool, [to_bs2]:bool, [to_bs8,to_bs9]:bool,

        (to_n0):u8, (to_n1):u16, (to_n2):u32, (to_n3):u64, (to_n4):u128, (to_n5):f32, (to_n6):f64,
        (to_t5, to_t6):TamponS2, (to_t7, to_t8):TamponS1, (to_t9):TamponS2,

        [to_ns0]:u8, [to_ns1]:u16, [to_ns2]:u32, [to_ns3]:u64, [to_ns4]:u128, [to_ns5]:f32, [to_ns6]:f64,
        [to_ns7]:i8, [to_ns8]:i16, [to_ns9]:i32, [to_ns10]:i64, [to_ns11]:i128, [to_bs3,to_bs4,to_bs5,to_bs6,to_bs7]:bool,

        (to_s5):String, (to_s6, to_s7, to_s8, to_s9):String,

        [to_ss0, to_ss1]:String, [to_ss2, to_ss3, to_ss4]:String, [to_ss5]:String, 
        [to_ss6, to_ss7, to_ss8, to_ss9]:String,

        (to_t0):TamponS1, (to_t1):TamponS2, (to_t2, to_t3, to_t4):TamponS1,
        (to_n7):i8, (to_n8):i16, (to_n9):i32, (to_n10):i64, (to_n11):i128,

        [to_ts0]:TamponS1, [to_ts1]:TamponS2, 
        [to_ts7, to_ts8]:TamponS1, [to_ts9]:TamponS2, (to_b8,to_b9):bool);

    // Get back data with deserialize!
    deserialize!(buffer, from_size, 
        [from_ts2, from_ts3, from_ts4]:TamponS1, [from_ts5, from_ts6]:TamponS2,
        (from_b0,from_b1):bool, (from_b2):bool, (from_b3,from_b4,from_b5, from_b6,from_b7):bool,
        (from_s0, from_s1):String, (from_s2, from_s3, from_s4):String,
        [from_bs0,from_bs1]:bool, [from_bs2]:bool, [from_bs8,from_bs9]:bool,

        (from_n0):u8, (from_n1):u16, (from_n2):u32, (from_n3):u64, (from_n4):u128, (from_n5):f32, (from_n6):f64,
        (from_t5, from_t6):TamponS2, (from_t7, from_t8):TamponS1, (from_t9):TamponS2,

        [from_ns0]:u8, [from_ns1]:u16, [from_ns2]:u32, [from_ns3]:u64, [from_ns4]:u128, [from_ns5]:f32, [from_ns6]:f64,
        [from_ns7]:i8, [from_ns8]:i16, [from_ns9]:i32, [from_ns10]:i64, [from_ns11]:i128, [from_bs3,from_bs4,from_bs5, from_bs6,from_bs7]:bool,

        (from_s5):String, (from_s6, from_s7, from_s8, from_s9):String,
        [from_ss0, from_ss1]:String, [from_ss2, from_ss3, from_ss4]:String, [from_ss5]:String, 
        [from_ss6, from_ss7, from_ss8, from_ss9]:String,

        (from_t0):TamponS1, (from_t1):TamponS2, (from_t2, from_t3, from_t4):TamponS1,
        (from_n7):i8, (from_n8):i16, (from_n9):i32, (from_n10):i64, (from_n11):i128,
        
        [from_ts0]:TamponS1, [from_ts1]:TamponS2,
        [from_ts7, from_ts8]:TamponS1, [from_ts9]:TamponS2, (from_b8,from_b9):bool);

    // All size should be the same
    println!("SIZE | VAR={} | TO={} | FROM={}", var_size, to_size, from_size);
    assert!(var_size == to_size && to_size == from_size);

    // Compare results of serialize! VS deserialize!
    assert!(
        from_b0 == to_b0 && from_b1 == to_b1 && from_b2 == to_b2 && from_b3 == to_b3 && from_b4 == to_b4
        && from_b5 == to_b5 && from_b6 == to_b6 && from_b7 == to_b7 && from_b8 == to_b8 && from_b9 == to_b9
        
        && do_vecs_match(&from_bs0, &to_bs0) && do_vecs_match(&from_bs1, &to_bs1) && do_vecs_match(&from_bs2, &to_bs2)
        && do_vecs_match(&from_bs3, &to_bs3) && do_vecs_match(&from_bs4, &to_bs4) && do_vecs_match(&from_bs5, &to_bs5)
        && do_vecs_match(&from_bs6, &to_bs6) && do_vecs_match(&from_bs7, &to_bs7) && do_vecs_match(&from_bs8, &to_bs8)
        && do_vecs_match(&from_bs9, &to_bs9)

        && from_n0 == to_n0 && from_n1 == to_n1 && from_n2 == to_n2 && from_n3 == to_n3
        && from_n4 == to_n4 && from_n5 == to_n5 && from_n6 == to_n6 && from_n7 == to_n7
        && from_n8 == to_n8 && from_n9 == to_n9 && from_n10 == to_n10 && from_n11 == to_n11

        && do_vecs_match(&from_ns0, &to_ns0) && do_vecs_match(&from_ns1, &to_ns1) && do_vecs_match(&from_ns2, &to_ns2)
        && do_vecs_match(&from_ns3, &to_ns3) && do_vecs_match(&from_ns4, &to_ns4) && do_vecs_match(&from_ns5, &to_ns5)
        && do_vecs_match(&from_ns6, &to_ns6) && do_vecs_match(&from_ns7, &to_ns7) && do_vecs_match(&from_ns8, &to_ns8)
        && do_vecs_match(&from_ns9, &to_ns9) && do_vecs_match(&from_ns10, &to_ns10) && do_vecs_match(&from_ns11, &to_ns11)

        && from_s0.eq(&to_s0) && from_s1.eq(&to_s1) && from_s2.eq(&to_s2) && from_s3.eq(&to_s3) && from_s4.eq(&to_s4)
        && from_s5.eq(&to_s5) && from_s6.eq(&to_s6) && from_s7.eq(&to_s7) && from_s8.eq(&to_s8) && from_s9.eq(&to_s9)

        && do_vecs_eq_match(&from_ss0, &to_ss0) && do_vecs_eq_match(&from_ss1, &to_ss1) && do_vecs_eq_match(&from_ss2, &to_ss2)
        && do_vecs_eq_match(&from_ss3, &to_ss3) && do_vecs_eq_match(&from_ss4, &to_ss4) && do_vecs_eq_match(&from_ss5, &to_ss5)
        && do_vecs_eq_match(&from_ss6, &to_ss6) && do_vecs_eq_match(&from_ss7, &to_ss7) && do_vecs_eq_match(&from_ss8, &to_ss8)
        && do_vecs_eq_match(&from_ss9, &to_ss9)

        && from_t0.eq(&to_t0) && from_t1.eq(&to_t1) && from_t2.eq(&to_t2) && from_t3.eq(&to_t3) && from_t4.eq(&to_t4)
        && from_t5.eq(&to_t5) && from_t6.eq(&to_t6) && from_t7.eq(&to_t7) && from_t8.eq(&to_t8) && from_t9.eq(&to_t9)

        && do_vecs_eq_match(&from_ts0, &to_ts0) && do_vecs_eq_match(&from_ts1, &to_ts1) && do_vecs_eq_match(&from_ts2, &to_ts2)
        && do_vecs_eq_match(&from_ts3, &to_ts3) && do_vecs_eq_match(&from_ts4, &to_ts4) && do_vecs_eq_match(&from_ts5, &to_ts5)
        && do_vecs_eq_match(&from_ts6, &to_ts6) && do_vecs_eq_match(&from_ts7, &to_ts7) && do_vecs_eq_match(&from_ts8, &to_ts8)
        && do_vecs_eq_match(&from_ts9, &to_ts9));
    println!("Value retrieved successfully!");
}