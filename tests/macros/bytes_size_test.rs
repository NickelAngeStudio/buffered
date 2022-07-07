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
    let size = n.get_size(1);

    assert!(macro_test_validation(size,
        bytes_size!(N(n.n1))
    ));
}


#[test]
// Test bytes size of 2 numeric type
fn bytes_size_2_numeric(){
    let n = DataNumerics::new();
    let size = n.get_size(2);

    assert!(macro_test_validation(size,
        bytes_size!(N(n.n1, n.n2))
    ));

}


#[test]
// Test bytes size of 10 numeric type
fn bytes_size_10_numeric(){

    let n = DataNumerics::new();
    let size = n.get_size(10);

    assert!(macro_test_validation(size,
        bytes_size!(N(n.n1, n.n2, n.n3, n.n4, n.n5, n.n6, n.n7, n.n8, n.n9, n.n10))
    ));

}

#[test]
// Test bytes size of 1 slice of numeric
fn bytes_size_1_numeric_slice(){

    let ns = DataNumericsSlices::new();
    let size = ns.get_size(1);

    assert!(macro_test_validation(size,
        bytes_size!(N[ns.ns1])
    ));

}

#[test]
// Test bytes size of 2 slice of numeric
fn bytes_size_2_numeric_slice(){

    let ns = DataNumericsSlices::new();
    let size = ns.get_size(2);

    assert!(macro_test_validation(size,
        bytes_size!(N[ns.ns1, ns.ns2])
    ));

}

#[test]
// Test bytes size of 10 slice of numeric
fn bytes_size_10_numeric_slice(){

    let ns = DataNumericsSlices::new();
    let size = ns.get_size(10);
    assert!(macro_test_validation(size,
        bytes_size!(N[ns.ns1, ns.ns2, ns.ns3, ns.ns4, ns.ns5, ns.ns6, ns.ns7, ns.ns8, ns.ns9, ns.ns10])
    ));

}

#[test]
// Test bytes size 1 string
fn bytes_size_1_string(){
    let s = DataStrings::new();
    let size = s.get_size(1);

    assert!(macro_test_validation(size,
        bytes_size!(S(s.s1))
    ));
}

#[test]
// Test bytes size 2 strings
fn bytes_size_2_string(){
    let s = DataStrings::new();
    let size = s.get_size(2);

    assert!(macro_test_validation(size,
        bytes_size!(S(s.s1, s.s2))
    ));
}

#[test]
// Test bytes size 10 strings
fn bytes_size_10_string(){
    let s = DataStrings::new();
    let size = s.get_size(10);

    assert!(macro_test_validation(size,
        bytes_size!(S(s.s1, s.s2, s.s3, s.s4, s.s5, s.s6, s.s7, s.s8, s.s9, s.s10))
    ));
}


#[test]
// Test bytes size 1 string slice
fn bytes_size_1_string_slice(){
    let ss = DataStringsSlices::new();
    let size = ss.get_size(1);

    assert!(macro_test_validation(size,
        bytes_size!(S[ss.ss1])
    ));
}

#[test]
// Test bytes size 2 strings slices
fn bytes_size_2_string_slice(){
    let ss = DataStringsSlices::new();
    let size = ss.get_size(2);

    assert!(macro_test_validation(size,
        bytes_size!(S[ss.ss1, ss.ss2])
    ));
}

#[test]
// Test bytes size 10 strings slices
fn bytes_size_10_string_slice(){
    let ss = DataStringsSlices::new();
    let size = ss.get_size(10);

    assert!(macro_test_validation(size,
        bytes_size!(S[ss.ss1, ss.ss2, ss.ss3, ss.ss4, ss.ss5, ss.ss6, ss.ss7, ss.ss8, ss.ss9, ss.ss10])
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
    let size = t.get_size(1);

    assert!(macro_test_validation(size,
        bytes_size!(T(t.t1))
    ));


}

#[test]
// Test bytes size of 2 implementor of Tampon trait
fn bytes_size_2_tampon(){

    let t = DataTampons::new();
    let size = t.get_size(2);

    assert!(macro_test_validation(size,
        bytes_size!(T(t.t1, t.t2))
    ));


}


#[test]
// Test bytes size of 10 implementor of Tampon trait
fn bytes_size_10_tampon(){

    let t = DataTampons::new();
    let size = t.get_size(10);

    assert!(macro_test_validation(size,
        bytes_size!(T(t.t1, t.t2, t.t3, t.t4, t.t5, t.t6, t.t7, t.t8, t.t9, t.t10))
    ));
}


#[test]
// Test bytes size of 1 slice of implementor of Tampon trait
fn bytes_size_1_tampon_slice(){

    let ts = DataTamponsSlices::new();
    let size = ts.get_size(1);

    assert!(macro_test_validation(size,
        bytes_size!(T[ts.ts1])
    ));

}

#[test]
// Test bytes size of 2 slice of implementor of Tampon trait
fn bytes_size_2_tampon_slice(){

    let ts = DataTamponsSlices::new();
    let size = ts.get_size(2);

    assert!(macro_test_validation(size,
        bytes_size!(T[ts.ts1, ts.ts2])
    ));

}

#[test]
// Test bytes size of 10 slice of implementor of Tampon trait
fn bytes_size_10_tampon_slice(){
    

    let ts = DataTamponsSlices::new();
    let size = ts.get_size(10);

    assert!(macro_test_validation(size,
        bytes_size!(T[ts.ts1, ts.ts2, ts.ts3, ts.ts4, ts.ts5, ts.ts6, ts.ts7, ts.ts8, ts.ts9, ts.ts10])
    ));

}


#[test]
// Test bytes size of 1n, 1s, 1t
fn bytes_size_1n_1s_1t(){

    let n = DataNumerics::new();
    let s = DataStrings::new();
    let t = DataTampons::new();
    let size = n.get_size(1) + s.get_size(1) + t.get_size(1);

    assert!(macro_test_validation(size,
        bytes_size!(N(n.n1), S(s.s1), T(t.t1))
    ));

}

#[test]
// Test bytes size of 1ns, 1ss, 1ts
fn bytes_size_1ns_1ss_1ts(){
    let ns = DataNumericsSlices::new();
    let ss = DataStringsSlices::new();
    let ts = DataTamponsSlices::new();
    let size = ns.get_size(1) + ss.get_size(1) + ts.get_size(1);

    assert!(macro_test_validation(size,
        bytes_size!(N[ns.ns1], S[ss.ss1], T[ts.ts1])
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
    let size = n.get_size(1) + s.get_size(1) + t.get_size(1)+ ns.get_size(1) + ss.get_size(1) + ts.get_size(1);

    assert!(macro_test_validation(size,
        bytes_size!(N(n.n1), S(s.s1), T(t.t1), N[ns.ns1], S[ss.ss1], T[ts.ts1])
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
    let size = n.get_size(2) + s.get_size(2) + t.get_size(2)+ ns.get_size(2) + ss.get_size(2) + ts.get_size(2);

    assert!(macro_test_validation(size,
        bytes_size!(
            N(n.n1, n.n2),
            S(s.s1, s.s2),
            N[ns.ns1, ns.ns2],
            T(t.t1, t.t2),
            S[ss.ss1, ss.ss2],
            T[ts.ts1, ts.ts2]
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
    let size = n.get_size(10) + s.get_size(10) + t.get_size(10)+ ns.get_size(10) + ss.get_size(10) + ts.get_size(10);

    assert!(macro_test_validation(size,
        bytes_size!(
            N(n.n1, n.n2, n.n3, n.n4, n.n5),
            T(t.t1, t.t2, t.t3, t.t4, t.t5),
            S(s.s1, s.s2, s.s3, s.s4, s.s5),
            N[ns.ns6, ns.ns7, ns.ns8, ns.ns9, ns.ns10],
            S[ss.ss10, ss.ss9, ss.ss8, ss.ss7, ss.ss6],
            T[ts.ts1, ts.ts2, ts.ts3, ts.ts4, ts.ts5],

            N(n.n6, n.n7, n.n8, n.n9, n.n10),
            T(t.t6, t.t7, t.t8, t.t9, t.t10),
            S(s.s10, s.s9, s.s8, s.s7, s.s6),
            N[ns.ns1, ns.ns2, ns.ns3, ns.ns4, ns.ns5],
            S[ss.ss5, ss.ss4, ss.ss3, ss.ss2, ss.ss1],
            T[ts.ts6, ts.ts7, ts.ts8, ts.ts9, ts.ts10]
        )
    ));
}
