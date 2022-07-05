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
use crate::{data::{ DataNumerics, macro_test_validation, DataNumericsSlices, DataTampons, DataTamponsSlices, DataStrings, DataStringsSlices}, implementation::TestStruct};


#[test]
// Test bytes size 1 numeric type
fn bytes_size_1_numeric(){
    let n = DataNumerics::new();
    assert!(macro_test_validation(n.get_size(1),
        bytes_size!(n(n.n1, u8))
    ));
}

#[test]
// Test bytes size of 2 numeric type
fn bytes_size_2_numeric(){
    let n = DataNumerics::new();
    assert!(macro_test_validation(n.get_size(2),
        bytes_size!(n(n.n1, u8), n(n.n2, u16))
    ));
}

#[test]
// Test bytes size of 10 numeric type
fn bytes_size_10_numeric(){

    let n = DataNumerics::new();
    assert!(macro_test_validation(n.get_size(10),
        bytes_size!(n(n.n1, u8), n(n.n2, u16),n(n.n3, u32), n(n.n4, u64),n(n.n5, u128),
            n(n.n6, f32), n(n.n7, f64),n(n.n8, i8), n(n.n9, i16),n(n.n10, i32))
    ));

}


#[test]
// Test bytes size of 1 slice of numeric
fn bytes_size_1_numeric_slice(){

    let ns = DataNumericsSlices::new();
    assert!(macro_test_validation(ns.get_size(1),
        bytes_size!(ns(ns.ns1, u8))
    ));

}

#[test]
// Test bytes size of 2 slice of numeric
fn bytes_size_2_numeric_slice(){

    let ns = DataNumericsSlices::new();
    assert!(macro_test_validation(ns.get_size(2),
        bytes_size!(ns(ns.ns1, u8), ns(ns.ns2, u16))
    ));

}

#[test]
// Test bytes size of 10 slice of numeric
fn bytes_size_10_numeric_slice(){

    let ns = DataNumericsSlices::new();
    assert!(macro_test_validation(ns.get_size(10),
        bytes_size!(ns(ns.ns1, u8), ns(ns.ns2, u16), ns(ns.ns3, u32), ns(ns.ns4, u64), ns(ns.ns5, u128),
        ns(ns.ns6, f32), ns(ns.ns7, f64), ns(ns.ns8, i8), ns(ns.ns9, i16), ns(ns.ns10, i32))
    ));

}


// TODO: String test

#[test]
// Test bytes size 1 string
fn bytes_size_1_string(){
    let s = DataStrings::new();

    assert!(macro_test_validation(s.get_size(1),
        bytes_size!(s(s.s1, String))
    ));
}

#[test]
// Test bytes size 2 strings
fn bytes_size_2_string(){
    let s = DataStrings::new();

    assert!(macro_test_validation(s.get_size(2),
        bytes_size!(s(s.s1, String),s(s.s2, String))
    ));
}

#[test]
// Test bytes size 10 strings
fn bytes_size_10_string(){
    let s = DataStrings::new();

    assert!(macro_test_validation(s.get_size(10),
        bytes_size!(s(s.s1, String),s(s.s2, String),s(s.s3, String),s(s.s4, String),s(s.s5, String),
            s(s.s6, String),s(s.s7, String),s(s.s8, String),s(s.s9, String),s(s.s10, String))
    ));
}


#[test]
// Test bytes size 1 string slice
fn bytes_size_1_string_slice(){
    let ss = DataStringsSlices::new();

    assert!(macro_test_validation(ss.get_size(1),
        bytes_size!(ss(ss.ss1, String))
    ));
}

#[test]
// Test bytes size 2 strings slices
fn bytes_size_2_string_slice(){
    let ss = DataStringsSlices::new();

    assert!(macro_test_validation(ss.get_size(2),
        bytes_size!(ss(ss.ss1, String),ss(ss.ss2, String))
    ));
}

#[test]
// Test bytes size 10 strings slices
fn bytes_size_10_string_slice(){
    let ss = DataStringsSlices::new();

    assert!(macro_test_validation(ss.get_size(10),
        bytes_size!(ss(ss.ss1, String),ss(ss.ss2, String),ss(ss.ss3, String),ss(ss.ss4, String),ss(ss.ss5, String),
            ss(ss.ss6, String),ss(ss.ss7, String),ss(ss.ss8, String),ss(ss.ss9, String),ss(ss.ss10, String))
    ));
}





#[test]
// Test that show that core::mem::size_of::<TestStruct>() != TestStruct::bytes_size()
fn bytes_size_struct_size_of_diff(){

    let t = DataTampons::new();

    let sizeof_size = core::mem::size_of::<TestStruct>();
    let bs = t.get_size(1);

    println!("size_of={}, bytes_size={}",sizeof_size, bs);

    assert!(sizeof_size != bs);

}

#[test]
// Test bytes size of 1 implementor of Tampon trait
fn bytes_size_1_tampon(){

    let t = DataTampons::new();
    assert!(macro_test_validation(t.get_size(1),
        bytes_size!(t(t.t1, TestStruct))
    ));


}

#[test]
// Test bytes size of 2 implementor of Tampon trait
fn bytes_size_2_tampon(){

    let t = DataTampons::new();
    assert!(macro_test_validation(t.get_size(2),
        bytes_size!(t(t.t1, TestStruct), t(t.t2, TestStruct))
    ));


}


#[test]
// Test bytes size of 10 implementor of Tampon trait
fn bytes_size_10_tampon(){

    let t = DataTampons::new();
    assert!(macro_test_validation(t.get_size(10),
        bytes_size!(t(t.t1, TestStruct), t(t.t2, TestStruct),t(t.t3, TestStruct), t(t.t4, TestStruct),t(t.t5, TestStruct),
        t(t.t6, TestStruct), t(t.t7, TestStruct),t(t.t8, TestStruct), t(t.t9, TestStruct),t(t.t10, TestStruct))
    ));



}


#[test]
// Test bytes size of 1 slice of implementor of Tampon trait
fn bytes_size_1_tampon_slice(){

    let ts = DataTamponsSlices::new();
    assert!(macro_test_validation(ts.get_size(1),
        bytes_size!(ts(ts.ts1, TestStruct))
    ));

}

#[test]
// Test bytes size of 2 slice of implementor of Tampon trait
fn bytes_size_2_tampon_slice(){

    let ts = DataTamponsSlices::new();
    assert!(macro_test_validation(ts.get_size(2),
        bytes_size!(ts(ts.ts1, TestStruct), ts(ts.ts2, TestStruct))
    ));

}

#[test]
// Test bytes size of 10 slice of implementor of Tampon trait
fn bytes_size_10_tampon_slice(){
    

    let ts = DataTamponsSlices::new();
    assert!(macro_test_validation(ts.get_size(10),
        bytes_size!(ts(ts.ts1, TestStruct), ts(ts.ts2, TestStruct),ts(ts.ts3, TestStruct), ts(ts.ts4, TestStruct),ts(ts.ts5, TestStruct),
        ts(ts.ts6, TestStruct), ts(ts.ts7, TestStruct),ts(ts.ts8, TestStruct), ts(ts.ts9, TestStruct),ts(ts.ts10, TestStruct))
    ));

}


#[test]
// Test bytes size of 1n, 1s, 1t
fn bytes_size_1n_1s_1t(){

    let n = DataNumerics::new();
    let s = DataStrings::new();
    let t = DataTampons::new();
    assert!(macro_test_validation(n.get_size(1) + s.get_size(1) + t.get_size(1),
        bytes_size!(n(n.n1, u8), s(s.s1, String), t(t.t1, TestStruct))
    ));

}

#[test]
// Test bytes size of 1ns, 1ss, 1ts
fn bytes_size_1ns_1ss_1ts(){

    let ns = DataNumericsSlices::new();
    let ss = DataStringsSlices::new();
    let ts = DataTamponsSlices::new();
    assert!(macro_test_validation(ns.get_size(1) + ss.get_size(1) + ts.get_size(1),
        bytes_size!(ns(ns.ns1, u8), ss(ss.ss1, String), ts(ts.ts1, TestStruct))
    ));

}

#[test]
// Test bytes size of 1n, 1s, 1t, 1ns, 1ss, 1ts
fn bytes_size_1n_1s_1t_1ns_1ss_1ts(){

    let n = DataNumerics::new();
    let s = DataStrings::new();
    let t = DataTampons::new();
    let ns = DataNumericsSlices::new();
    let ss =DataStringsSlices::new();
    let ts = DataTamponsSlices::new();

    assert!(macro_test_validation(n.get_size(1) + s.get_size(1) + t.get_size(1)+ ns.get_size(1) + ss.get_size(1) + ts.get_size(1),
        bytes_size!(n(n.n1, u8), s(s.s1, String), t(t.t1, TestStruct),ns(ns.ns1, u8), ss(ss.ss1, String), ts(ts.ts1, TestStruct))
    ));

}

#[test]
// Test bytes size of 2n, 2s, 2t, 2ns, 2ss, 2ts
fn bytes_size_2n_2s_2t_2ns_2ss_2ts(){

    let n = DataNumerics::new();
    let s = DataStrings::new();
    let t = DataTampons::new();
    let ns = DataNumericsSlices::new();
    let ss =DataStringsSlices::new();
    let ts = DataTamponsSlices::new();

    assert!(macro_test_validation(n.get_size(2) + s.get_size(2) + t.get_size(2)+ ns.get_size(2) + ss.get_size(2) + ts.get_size(2),
        bytes_size!(
            n(n.n1, u8), n(n.n2, u16),
            s(s.s1, String), s(s.s2, String),
            ns(ns.ns1, u8), ns(ns.ns2, u16),
            t(t.t1, TestStruct), t(t.t2, TestStruct),
            ss(ss.ss1, String), ss(ss.ss2, String),
            ts(ts.ts1, TestStruct), ts(ts.ts2, TestStruct)
        )
    ));

}

#[test]
// Test bytes size of 10n, 10s, 10t, 10ns, 10ss, 10ts
fn bytes_size_10n_10s_10t_10ns_10ss_10ts(){

    let n = DataNumerics::new();
    let s = DataStrings::new();
    let t = DataTampons::new();
    let ns = DataNumericsSlices::new();
    let ss =DataStringsSlices::new();
    let ts = DataTamponsSlices::new();

    assert!(macro_test_validation(n.get_size(10) + s.get_size(10) + t.get_size(10)+ ns.get_size(10) + ss.get_size(10) + ts.get_size(10),
        bytes_size!(
            n(n.n1, u8), n(n.n2, u16),n(n.n3, u32), n(n.n4, u64),n(n.n5, u128),
            t(t.t1, TestStruct), t(t.t2, TestStruct),t(t.t3, TestStruct), t(t.t4, TestStruct),t(t.t5, TestStruct),
            s(s.s1, String), s(s.s2, String),s(s.s3, String), s(s.s4, String),s(s.s5, String),
            ns(ns.ns6, f32), ns(ns.ns7, f64), ns(ns.ns8, i8), ns(ns.ns9, i16), ns(ns.ns10, i32),
            ss(ss.ss10, String), ss(ss.ss9, String), ss(ss.ss8, String), ss(ss.ss7, String), ss(ss.ss6, String),
            ts(ts.ts1, TestStruct), ts(ts.ts2, TestStruct),ts(ts.ts3, TestStruct), ts(ts.ts4, TestStruct),ts(ts.ts5, TestStruct),

            n(n.n6, f32), n(n.n7, f64),n(n.n8, i8), n(n.n9, i16),n(n.n10, i32),
            t(t.t6, TestStruct), t(t.t7, TestStruct),t(t.t8, TestStruct), t(t.t9, TestStruct),t(t.t10, TestStruct),
            s(s.s10, String), s(s.s9, String),s(s.s8, String), s(s.s7, String),s(s.s6, String),
            ns(ns.ns1, u8), ns(ns.ns2, u16), ns(ns.ns3, u32), ns(ns.ns4, u64), ns(ns.ns5, u128),
            ss(ss.ss5, String), ss(ss.ss4, String), ss(ss.ss3, String), ss(ss.ss2, String), ss(ss.ss1, String),
            ts(ts.ts6, TestStruct), ts(ts.ts7, TestStruct),ts(ts.ts8, TestStruct), ts(ts.ts9, TestStruct),ts(ts.ts10, TestStruct)
        )
    ));
}