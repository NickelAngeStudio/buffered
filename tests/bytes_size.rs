/*
 * @file tampon/tests/bytes_size.rs
 *
 * @module tampon::tests
 *
 * @brief Contains unit and integration tests for bytes_size! macro.
 * 
 * @details
 * Contains unit and integration tests for bytes_size! macro.
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

use tampon::{bytes_size, Tampon};
use crate::tampon_impl::{ TestStruct, vec_of_test_struct } ;

// Implementation of tampon
mod tampon_impl;

// Size of slices
static SLICE_SIZE: usize = 255;


#[test]
// Test bytes size of 1 primitive
fn bytes_size_1_primitive(){
    let _p1 = 0;

    let size = bytes_size!(p(_p1, i32));
    let expected = core::mem::size_of::<i32>();

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);
}

#[test]
// Test bytes size of 2 primitive
fn bytes_size_2_primitive(){
    let _p1 :u8 = 0;let _p2:u16 = 12;

    let size = bytes_size!(p(_p1, u8), p(_p2, u16));
    let expected = core::mem::size_of::<u8>() + core::mem::size_of::<u16>();

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);
}

#[test]
// Test bytes size of 10 primitive
fn bytes_size_10_primitive(){

    let _p1 :u8 = 0;let _p2:u16 = 12;let _p3 :u32 = 100000;let _p4:u64 = 1000000000;let _p5 :u128 = 1000000000000;
    let _p6 :f32 = 125.55;let _p7:f64 = 123456.58;let _p8 :bool = true;let _p9:char = 'g';let _p10 :i8 = -25;

    let size = bytes_size!(p(_p1, u8), p(_p2, u16),p(_p3, u32), p(_p4, u64),p(_p5, u128),
            p(_p6, f32), p(_p7, f64),p(_p8, bool), p(_p9, char),p(_p10, i8));

    let expected = core::mem::size_of::<u8>() + core::mem::size_of::<u16>() + core::mem::size_of::<u32>() +
    core::mem::size_of::<u64>() + core::mem::size_of::<u128>() + core::mem::size_of::<f32>() + 
    core::mem::size_of::<f64>() + core::mem::size_of::<bool>() + core::mem::size_of::<char>() +
    core::mem::size_of::<i8>();

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);
}

#[test]
// Test bytes size of 1 slice of primitive
fn bytes_size_1_primitive_slice(){

    let _ps1: Vec<u8> = vec![5;SLICE_SIZE];

    let size = bytes_size!(ps(_ps1, u8));
    let expected = core::mem::size_of::<u8>() * SLICE_SIZE ;

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}

#[test]
// Test bytes size of 2 slice of primitive
fn bytes_size_2_primitive_slice(){

    let _ps1: Vec<u8> = vec![5;SLICE_SIZE];let _ps2: Vec<u16> = vec![60000;SLICE_SIZE];

    let size = bytes_size!(ps(_ps1, u8), ps(_ps2, u16));
    let expected = core::mem::size_of::<u8>() * SLICE_SIZE + core::mem::size_of::<u16>() * SLICE_SIZE;

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);


}

#[test]
// Test bytes size of 10 slice of primitive
fn bytes_size_10_primitive_slice(){

    let _ps1: Vec<u8> = vec![u8::MAX;SLICE_SIZE];let _ps2: Vec<u16> = vec![u16::MAX;SLICE_SIZE];let _ps3: Vec<u32> = vec![u32::MAX;SLICE_SIZE];
    let _ps4: Vec<u64> = vec![u64::MAX;SLICE_SIZE];let _ps5: Vec<u128> = vec![u128::MAX;SLICE_SIZE];let _ps6: Vec<f32> = vec![f32::MAX;SLICE_SIZE];
    let _ps7: Vec<f64> = vec![f64::MAX;SLICE_SIZE];let _ps8: Vec<bool> = vec![true;SLICE_SIZE];let _ps9: Vec<char> = vec!['x';SLICE_SIZE];
    let _ps10: Vec<i8> = vec![i8::MIN;SLICE_SIZE];

    let size = bytes_size!(ps(_ps1, u8), ps(_ps2, u16), ps(_ps3, u32), ps(_ps4, u64), ps(_ps5, u128),
    ps(_ps6, f32), ps(_ps7, f64), ps(_ps8, bool), ps(_ps9, char), ps(_ps10, i8));

    let expected = core::mem::size_of::<u8>() * SLICE_SIZE + core::mem::size_of::<u16>() * SLICE_SIZE +
    core::mem::size_of::<u32>() * SLICE_SIZE + core::mem::size_of::<u64>() * SLICE_SIZE +
    core::mem::size_of::<u128>() * SLICE_SIZE + core::mem::size_of::<f32>() * SLICE_SIZE +
    core::mem::size_of::<f64>() * SLICE_SIZE + core::mem::size_of::<bool>() * SLICE_SIZE +
    core::mem::size_of::<char>() * SLICE_SIZE + core::mem::size_of::<i8>() * SLICE_SIZE;

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}

#[test]
// Test that show that core::mem::size_of::<TestStruct>() != TestStruct::bytes_size()
fn bytes_size_struct_size_of_diff(){

    let _t1: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, SLICE_SIZE);

    let sizeof_size = core::mem::size_of::<TestStruct>();
    let bs = _t1.bytes_size() ;

    println!("size_of={}, bytes_size={}",sizeof_size, bs);

    assert!(sizeof_size != bs);

}

#[test]
// Test bytes size of 1 implementor of Tampon trait
fn bytes_size_1_tampon(){

    let _t1: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, SLICE_SIZE);

    let size = bytes_size!(t(_t1, TestStruct));
    let expected = _t1.bytes_size() ;

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}

#[test]
// Test bytes size of 2 implementor of Tampon trait
fn bytes_size_2_tampon(){

    let _t1: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 10);
    let _t2: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 20);

    let size = bytes_size!(t(_t1, TestStruct), t(_t2, TestStruct));
    let expected = _t1.bytes_size() + _t2.bytes_size() ;

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}


#[test]
// Test bytes size of 10 implementor of Tampon trait
fn bytes_size_10_tampon(){

    let _t1: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 10);
    let _t2: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 20);
    let _t3: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 30);
    let _t4: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 40);
    let _t5: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 50);
    let _t6: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 60);
    let _t7: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 70);
    let _t8: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 80);
    let _t9: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 90);
    let _t10: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 100);

    let size = bytes_size!(t(_t1, TestStruct), t(_t2, TestStruct),t(_t3, TestStruct), t(_t4, TestStruct),t(_t5, TestStruct),
    t(_t6, TestStruct), t(_t7, TestStruct),t(_t8, TestStruct), t(_t9, TestStruct),t(_t10, TestStruct));

    let expected = _t1.bytes_size() + _t2.bytes_size() + _t3.bytes_size() + _t4.bytes_size() + _t5.bytes_size() +
    _t6.bytes_size() + _t7.bytes_size() + _t8.bytes_size() + _t9.bytes_size() + _t10.bytes_size();

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}


#[test]
// Test bytes size of 1 slice of implementor of Tampon trait
fn bytes_size_1_tampon_slice(){

    let _ts1 = vec_of_test_struct(10);

    let size = bytes_size!(ts(_ts1, TestStruct));
    let expected = _ts1[0].bytes_size() * _ts1.len() ;

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}

#[test]
// Test bytes size of 2 slice of implementor of Tampon trait
fn bytes_size_2_tampon_slice(){

    let _ts1 = vec_of_test_struct(10);
    let _ts2 = vec_of_test_struct(20);

    let size = bytes_size!(ts(_ts1, TestStruct), ts(_ts2, TestStruct));
    let expected = _ts1[0].bytes_size() * _ts1.len() + _ts2[0].bytes_size() * _ts2.len() ;

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}

#[test]
// Test bytes size of 10 slice of implementor of Tampon trait
fn bytes_size_10_tampon_slice(){

    let _ts1 = vec_of_test_struct(10);
    let _ts2 = vec_of_test_struct(20);
    let _ts3 = vec_of_test_struct(30);
    let _ts4 = vec_of_test_struct(40);
    let _ts5 = vec_of_test_struct(50);
    let _ts6 = vec_of_test_struct(60);
    let _ts7 = vec_of_test_struct(70);
    let _ts8 = vec_of_test_struct(80);
    let _ts9 = vec_of_test_struct(90);
    let _ts10 = vec_of_test_struct(100);
    

    let size = bytes_size!(ts(_ts1, TestStruct), ts(_ts2, TestStruct),ts(_ts3, TestStruct), ts(_ts4, TestStruct),ts(_ts5, TestStruct),
    ts(_ts6, TestStruct), ts(_ts7, TestStruct),ts(_ts8, TestStruct), ts(_ts9, TestStruct),ts(_ts10, TestStruct));

    let expected = _ts1[0].bytes_size() * _ts1.len() + _ts2[0].bytes_size() * _ts2.len() +
    _ts3[0].bytes_size() * _ts3.len() + _ts4[0].bytes_size() * _ts4.len() +
    _ts5[0].bytes_size() * _ts5.len() + _ts6[0].bytes_size() * _ts6.len() +
    _ts7[0].bytes_size() * _ts7.len() + _ts8[0].bytes_size() * _ts8.len() +
    _ts9[0].bytes_size() * _ts9.len() + _ts10[0].bytes_size() * _ts10.len();

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}


#[test]
// Test bytes size of 1 p, 1t
fn bytes_size_1p_1t(){

    let _p1 :u8 = 0;
    let _t1: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 10);

    let size = bytes_size!(p(_p1, u8), t(_t1, TestStruct));
    let expected = core::mem::size_of::<u8>() + _t1.bytes_size();

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}

#[test]
// Test bytes size of 1ps, 1ts
fn bytes_size_1ps_1ts(){

    let _ps1: Vec<u8> = vec![5;SLICE_SIZE];
    let _ts1 = vec_of_test_struct(10);

    let size = bytes_size!(ps(_ps1, u8), ts(_ts1, TestStruct));
    let expected = core::mem::size_of::<u8>() * SLICE_SIZE + _ts1[0].bytes_size() * _ts1.len() ;

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}

#[test]
// Test bytes size of 1p, 1t, 1ps, 1ts
fn bytes_size_1p_1t_1ps_1ts(){

    let _p1 :u8 = 0;
    let _t1: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 10);
    let _ps1: Vec<u8> = vec![5;SLICE_SIZE];
    let _ts1 = vec_of_test_struct(10);

    let size = bytes_size!(p(_p1, u8), t(_t1, TestStruct),ps(_ps1, u8), ts(_ts1, TestStruct));
    let expected = core::mem::size_of::<u8>() + _t1.bytes_size() +
    core::mem::size_of::<u8>() * SLICE_SIZE + _ts1[0].bytes_size() * _ts1.len() ;

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}

#[test]
// Test bytes size of 2p, 2t, 2ps, 2ts
fn bytes_size_2p_2t_2ps_2ts(){
    let _p1 :u8 = 0;let _p2:u16 = 12;

    let _t1: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 10);
    let _t2: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 20);

    let _ps1: Vec<u8> = vec![5;SLICE_SIZE];let _ps2: Vec<u16> = vec![60000;SLICE_SIZE];

    let _ts1 = vec_of_test_struct(10);
    let _ts2 = vec_of_test_struct(20);


    let size = bytes_size!(
        p(_p1, u8), p(_p2, u16),
        ps(_ps1, u8), ps(_ps2, u16),
        t(_t1, TestStruct), t(_t2, TestStruct),
        ts(_ts1, TestStruct), ts(_ts2, TestStruct)
    );
    let expected = core::mem::size_of::<u8>() + core::mem::size_of::<u16>() +
    core::mem::size_of::<u8>() * SLICE_SIZE + core::mem::size_of::<u16>() * SLICE_SIZE +
    _t1.bytes_size() + _t2.bytes_size() +
    _ts1[0].bytes_size() * _ts1.len() + _ts2[0].bytes_size() * _ts2.len();


    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}

#[test]
// Test bytes size of 10p, 10t, 10ps, 10ts
fn bytes_size_10p_10t_10ps_10ts(){

    let _p1 :u8 = 0;let _p2:u16 = 12;let _p3 :u32 = 100000;let _p4:u64 = 1000000000;let _p5 :u128 = 1000000000000;
    let _p6 :f32 = 125.55;let _p7:f64 = 123456.58;let _p8 :bool = true;let _p9:char = 'g';let _p10 :i8 = -25;

    let _t1: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 10);
    let _t2: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 20);
    let _t3: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 30);
    let _t4: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 40);
    let _t5: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 50);
    let _t6: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 60);
    let _t7: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 70);
    let _t8: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 80);
    let _t9: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 90);
    let _t10: TestStruct = TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 100);

    let _ps1: Vec<u8> = vec![u8::MAX;SLICE_SIZE];let _ps2: Vec<u16> = vec![u16::MAX;SLICE_SIZE];let _ps3: Vec<u32> = vec![u32::MAX;SLICE_SIZE];
    let _ps4: Vec<u64> = vec![u64::MAX;SLICE_SIZE];let _ps5: Vec<u128> = vec![u128::MAX;SLICE_SIZE];let _ps6: Vec<f32> = vec![f32::MAX;SLICE_SIZE];
    let _ps7: Vec<f64> = vec![f64::MAX;SLICE_SIZE];let _ps8: Vec<bool> = vec![true;SLICE_SIZE];let _ps9: Vec<char> = vec!['x';SLICE_SIZE];
    let _ps10: Vec<i8> = vec![i8::MIN;SLICE_SIZE];

    let _ts1 = vec_of_test_struct(10);
    let _ts2 = vec_of_test_struct(20);
    let _ts3 = vec_of_test_struct(30);
    let _ts4 = vec_of_test_struct(40);
    let _ts5 = vec_of_test_struct(50);
    let _ts6 = vec_of_test_struct(60);
    let _ts7 = vec_of_test_struct(70);
    let _ts8 = vec_of_test_struct(80);
    let _ts9 = vec_of_test_struct(90);
    let _ts10 = vec_of_test_struct(100);

    let size = bytes_size!(
        p(_p1, u8), p(_p2, u16),p(_p3, u32), p(_p4, u64),p(_p5, u128),
        p(_p6, f32), p(_p7, f64),p(_p8, bool), p(_p9, char),p(_p10, i8),
        t(_t1, TestStruct), t(_t2, TestStruct),t(_t3, TestStruct), t(_t4, TestStruct),t(_t5, TestStruct),
        t(_t6, TestStruct), t(_t7, TestStruct),t(_t8, TestStruct), t(_t9, TestStruct),t(_t10, TestStruct),
        ps(_ps1, u8), ps(_ps2, u16), ps(_ps3, u32), ps(_ps4, u64), ps(_ps5, u128),
        ps(_ps6, f32), ps(_ps7, f64), ps(_ps8, bool), ps(_ps9, char), ps(_ps10, i8),
        ts(_ts1, TestStruct), ts(_ts2, TestStruct),ts(_ts3, TestStruct), ts(_ts4, TestStruct),ts(_ts5, TestStruct),
        ts(_ts6, TestStruct), ts(_ts7, TestStruct),ts(_ts8, TestStruct), ts(_ts9, TestStruct),ts(_ts10, TestStruct)
    );

    let expected = core::mem::size_of::<u8>() + core::mem::size_of::<u16>() + core::mem::size_of::<u32>() +
    core::mem::size_of::<u64>() + core::mem::size_of::<u128>() + core::mem::size_of::<f32>() + 
    core::mem::size_of::<f64>() + core::mem::size_of::<bool>() + core::mem::size_of::<char>() +
    core::mem::size_of::<i8>() + _t1.bytes_size() + _t2.bytes_size() + _t3.bytes_size() + _t4.bytes_size() + _t5.bytes_size() +
    _t6.bytes_size() + _t7.bytes_size() + _t8.bytes_size() + _t9.bytes_size() + _t10.bytes_size() +
    core::mem::size_of::<u8>() * SLICE_SIZE + core::mem::size_of::<u16>() * SLICE_SIZE +
    core::mem::size_of::<u32>() * SLICE_SIZE + core::mem::size_of::<u64>() * SLICE_SIZE +
    core::mem::size_of::<u128>() * SLICE_SIZE + core::mem::size_of::<f32>() * SLICE_SIZE +
    core::mem::size_of::<f64>() * SLICE_SIZE + core::mem::size_of::<bool>() * SLICE_SIZE +
    core::mem::size_of::<char>() * SLICE_SIZE + core::mem::size_of::<i8>() * SLICE_SIZE +
    _ts1[0].bytes_size() * _ts1.len() + _ts2[0].bytes_size() * _ts2.len() +
    _ts3[0].bytes_size() * _ts3.len() + _ts4[0].bytes_size() * _ts4.len() +
    _ts5[0].bytes_size() * _ts5.len() + _ts6[0].bytes_size() * _ts6.len() +
    _ts7[0].bytes_size() * _ts7.len() + _ts8[0].bytes_size() * _ts8.len() +
    _ts9[0].bytes_size() * _ts9.len() + _ts10[0].bytes_size() * _ts10.len();

    println!("Size={}, expected={}",size, expected);

    assert!(size == expected);

}
