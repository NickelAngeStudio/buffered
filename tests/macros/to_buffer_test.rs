/*
 * @file tampon/tests/to_buffer_test.rs
 *
 * @module tampon::tests
 *
 * @brief Contains unit and integration tests for to_buffer! macro.
 * 
 * @details
 * Contains unit and integration tests for to_buffer! macro.
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


use tampon::{to_buffer, compare_buffers};

use crate::{data::{DataNumerics, macro_test_validation, DataTampons, DataNumericsSlices, DataTamponsSlices}, implementation::TestStruct};




// TODO:Test without enough space

#[test]
// Test to_buffer with 1 numeric type
fn to_buffer_1_numeric(){
    let n = DataNumerics::new();
    let size = n.get_size(1);
    let mut buffer:Vec<u8> = vec![0;size];
    let expected:Vec<u8> = vec![255;1];

    assert!(macro_test_validation(size,
        to_buffer!(buffer, 0, N(n.n1))
    ));

    println!("Buffer={:?}\nExpected={:?}", buffer, expected);
    assert!(compare_buffers(&buffer, &expected) == 0);
    
}
 
#[test]
// Test to_buffer with 2 numeric type
fn to_buffer_2_numeric(){


    let n = DataNumerics::new();
    let size = n.get_size(2);
    let mut buffer:Vec<u8> = vec![0;size];
    let expected:Vec<u8> = vec![255;3];

  
    assert!(macro_test_validation(size,
        to_buffer!(buffer, 0, N(n.n1, n.n2))
    ));


    println!("Buffer={:?}\nExpected={:?}", buffer, expected);
    assert!(compare_buffers(&buffer, &expected) == 0);

}

#[test]
// Test to_buffer with 10 numeric type
fn to_buffer_10_numeric(){

    let n = DataNumerics::new();
    let size = n.get_size(10);
    let mut buffer:Vec<u8> = vec![0;size];
    let expected:Vec<u8> = vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 127, 127, 255, 255, 255, 255, 255, 255, 239, 127, 127, 255, 127, 255, 255, 255, 127];

  
    assert!(macro_test_validation(size,
        to_buffer!(buffer, 0, N(n.n1, n.n2, n.n3, n.n4, n.n5, n.n6, n.n7, n.n8, n.n9, n.n10))
    ));


    println!("Buffer={:?}\nExpected={:?}", buffer, expected);
    assert!(compare_buffers(&buffer, &expected) == 0);

}


#[test]
// Test to_buffer with 1 slice of numeric
fn to_buffer_1_numeric_slice(){

    let ns = DataNumericsSlices::new();
    let size = ns.get_size(1);
    let mut buffer:Vec<u8> = vec![0;size];
   
    assert!(macro_test_validation(size,
        to_buffer!(buffer, 0, N[ns.ns1])
    ));

    println!("Buffer={:?}", buffer);
}

/*
#[test]
// Test to_buffer with 2 slice of numeric
fn to_buffer_2_numeric_slice(){

    let ns = DataNumericsSlices::new();
    let size = ns.get_size(2);
    let mut buffer:Vec<u8> = vec![0;size];
   
    assert!(macro_test_validation(size,
        to_buffer!(buffer, 0, NS[ns.ns1, ns.ns2])
    ));

    println!("Buffer={:?}", buffer);
}
*/


/*
#[test]
// Test to_buffer of 1 primitive
fn to_buffer_1_primitive(){
    let p = DataNumerics::new();
    let mut buffer: Vec<u8> = vec![0;p.get_size(1)];
    
    assert!(macro_test_validation(p.get_size(1),
        to_buffer!(buffer, 0, p(p.n1, u8))
    ));

}

#[test]
// Test to_buffer of 2 primitive
fn to_buffer_2_primitive(){
    let p = DataNumerics::new();
    let mut buffer: Vec<u8> = vec![0;p.get_size(2)];

    assert!(macro_test_validation(p.get_size(2),
        to_buffer!(buffer, 0, p(p.n1, u8), p(p.n2, u16))
    ));
}

#[test]
// Test to_buffer of 10 primitive
fn to_buffer_10_primitive(){

    let p = DataNumerics::new();
    let mut buffer: Vec<u8> = vec![0;p.get_size(10)];

    assert!(macro_test_validation(p.get_size(10),
        to_buffer!(buffer, 0, p(p.n1, u8), p(p.n2, u16),p(p.n3, u32), p(p.n4, u64),p(p.n5, u128),
            p(p.n6, f32), p(p.n7, f64),p(p.n8, bool), p(p.n9, char),p(p.n10, i8))
    ));

}

#[test]
// Test to_buffer of 1 slice of primitive
fn to_buffer_1_primitive_slice(){

    let ps = DataNumericsSlices::new();
    let mut buffer: Vec<u8> = vec![0;ps.get_size(1)];

    assert!(macro_test_validation(ps.get_size(1),
        to_buffer!(buffer, 0, ps(ps.ns1, u8))
    ));

}

#[test]
// Test to_buffer of 2 slice of primitive
fn to_buffer_2_primitive_slice(){

    let ps = DataNumericsSlices::new();
    let mut buffer: Vec<u8> = vec![0;ps.get_size(2)];

    assert!(macro_test_validation(ps.get_size(2),
        to_buffer!(buffer, 0, ps(ps.ns1, u8), ps(ps.ns2, u16))
    ));

}

#[test]
// Test to_buffer of 10 slice of primitive
fn to_buffer_10_primitive_slice(){

    let ps = DataNumericsSlices::new();
    let mut buffer: Vec<u8> = vec![0;ps.get_size(10)];

    assert!(macro_test_validation(ps.get_size(10),
        to_buffer!(buffer, 0, ps(ps.ns1, u8), ps(ps.ns2, u16), ps(ps.ns3, u32), ps(ps.ns4, u64), ps(ps.ns5, u128),
        ps(ps.ns6, f32), ps(ps.ns7, f64), ps(ps.ns8, bool), ps(ps.ns9, char), ps(ps.ns10, i8))
    ));

}


#[test]
// Test to_buffer of 1 implementor of Tampon trait
fn to_buffer_1_tampon(){

    let t = DataTampons::new();
    let mut buffer: Vec<u8> = vec![0;t.get_size(1)];

    assert!(macro_test_validation(t.get_size(1),
        to_buffer!(buffer, 0, t(t.t1, TestStruct))
    ));


}

#[test]
// Test to_buffer of 2 implementor of Tampon trait
fn to_buffer_2_tampon(){

    let t = DataTampons::new();
    let mut buffer: Vec<u8> = vec![0;t.get_size(2)];

    assert!(macro_test_validation(t.get_size(2),
        to_buffer!(buffer, 0, t(t.t1, TestStruct), t(t.t2, TestStruct))
    ));


}


#[test]
// Test to_buffer of 10 implementor of Tampon trait
fn to_buffer_10_tampon(){

    let t = DataTampons::new();
    let mut buffer: Vec<u8> = vec![0;t.get_size(10)];

    assert!(macro_test_validation(t.get_size(10),
        to_buffer!(buffer, 0, t(t.t1, TestStruct), t(t.t2, TestStruct),t(t.t3, TestStruct), t(t.t4, TestStruct),t(t.t5, TestStruct),
        t(t.t6, TestStruct), t(t.t7, TestStruct),t(t.t8, TestStruct), t(t.t9, TestStruct),t(t.t10, TestStruct))
    ));



}


#[test]
// Test to_buffer of 1 slice of implementor of Tampon trait
fn to_buffer_1_tampon_slice(){

    let ts = DataTamponsSlices::new();
    let mut buffer: Vec<u8> = vec![0;ts.get_size(1)];

    assert!(macro_test_validation(ts.get_size(1),
        to_buffer!(buffer, 0, ts(ts.ts1, TestStruct))
    ));

}

#[test]
// Test to_buffer of 2 slice of implementor of Tampon trait
fn to_buffer_2_tampon_slice(){

    let ts = DataTamponsSlices::new();
    let mut buffer: Vec<u8> = vec![0;ts.get_size(2)];

    assert!(macro_test_validation(ts.get_size(2),
        to_buffer!(buffer, 0, ts(ts.ts1, TestStruct), ts(ts.ts2, TestStruct))
    ));

}

#[test]
// Test to_buffer of 10 slice of implementor of Tampon trait
fn to_buffer_10_tampon_slice(){
    

    let ts = DataTamponsSlices::new();
    let mut buffer: Vec<u8> = vec![0;ts.get_size(10)];

    assert!(macro_test_validation(ts.get_size(10),
        to_buffer!(buffer, 0, ts(ts.ts1, TestStruct), ts(ts.ts2, TestStruct),ts(ts.ts3, TestStruct), ts(ts.ts4, TestStruct),ts(ts.ts5, TestStruct),
        ts(ts.ts6, TestStruct), ts(ts.ts7, TestStruct),ts(ts.ts8, TestStruct), ts(ts.ts9, TestStruct),ts(ts.ts10, TestStruct))
    ));

}


#[test]
// Test to_buffer of 1 p, 1t
fn to_buffer_1p_1t(){

    let p = DataNumerics::new();
    let t = DataTampons::new();
    let mut buffer: Vec<u8> = vec![0;p.get_size(1) + t.get_size(1)];

    assert!(macro_test_validation(p.get_size(1) + t.get_size(1),
        to_buffer!(buffer, 0, p(p.n1, u8), t(t.t1, TestStruct))
    ));

}

#[test]
// Test to_buffer of 1ps, 1ts
fn to_buffer_1ps_1ts(){


    let ps = DataNumericsSlices::new();
    let ts = DataTamponsSlices::new();
    let mut buffer: Vec<u8> = vec![0;ps.get_size(1) + ts.get_size(1)];

    assert!(macro_test_validation(ps.get_size(1) + ts.get_size(1),
        to_buffer!(buffer, 0, ps(ps.ns1, u8), ts(ts.ts1, TestStruct))
    ));



}

#[test]
// Test to_buffer of 1p, 1t, 1ps, 1ts
fn to_buffer_1p_1t_1ps_1ts(){

    let p = DataNumerics::new();
    let t = DataTampons::new();
    let ps = DataNumericsSlices::new();
    let ts = DataTamponsSlices::new();
    let mut buffer: Vec<u8> = vec![0;p.get_size(1) + t.get_size(1)+ ps.get_size(1) + ts.get_size(1)];

    assert!(macro_test_validation(p.get_size(1) + t.get_size(1)+ ps.get_size(1) + ts.get_size(1),
        to_buffer!(buffer, 0, p(p.n1, u8), t(t.t1, TestStruct),ps(ps.ns1, u8), ts(ts.ts1, TestStruct))
    ));

}

#[test]
// Test to_buffer of 2p, 2t, 2ps, 2ts
fn to_buffer_2p_2t_2ps_2ts(){

    let p = DataNumerics::new();
    let t = DataTampons::new();
    let ps = DataNumericsSlices::new();
    let ts = DataTamponsSlices::new();
    let mut buffer: Vec<u8> = vec![0;p.get_size(2) + t.get_size(2)+ ps.get_size(2) + ts.get_size(2)];

    assert!(macro_test_validation(p.get_size(2) + t.get_size(2)+ ps.get_size(2) + ts.get_size(2),
        to_buffer!(buffer, 0, 
            p(p.n1, u8), p(p.n2, u16),
            ps(ps.ns1, u8), ps(ps.ns2, u16),
            t(t.t1, TestStruct), t(t.t2, TestStruct),
            ts(ts.ts1, TestStruct), ts(ts.ts2, TestStruct)
        )
    ));

}

#[test]
// Test to_buffer of 10p, 10t, 10ps, 10ts
fn to_buffer_10p_10t_10ps_10ts(){

    let p = DataNumerics::new();
    let t = DataTampons::new();
    let ps = DataNumericsSlices::new();
    let ts = DataTamponsSlices::new();
    let mut buffer: Vec<u8> = vec![0;p.get_size(10) + t.get_size(10)+ ps.get_size(10) + ts.get_size(10)];

    assert!(macro_test_validation(p.get_size(10) + t.get_size(10)+ ps.get_size(10) + ts.get_size(10),
        to_buffer!(buffer, 0, 
            p(p.n1, u8), p(p.n2, u16),p(p.n3, u32), p(p.n4, u64),p(p.n5, u128),
            t(t.t1, TestStruct), t(t.t2, TestStruct),t(t.t3, TestStruct), t(t.t4, TestStruct),t(t.t5, TestStruct),
            ps(ps.ns6, f32), ps(ps.ns7, f64), ps(ps.ns8, bool), ps(ps.ns9, char), ps(ps.ns10, i8),
            ts(ts.ts1, TestStruct), ts(ts.ts2, TestStruct),ts(ts.ts3, TestStruct), ts(ts.ts4, TestStruct),ts(ts.ts5, TestStruct),
            p(p.n6, f32), p(p.n7, f64),p(p.n8, bool), p(p.n9, char),p(p.n10, i8),
            t(t.t6, TestStruct), t(t.t7, TestStruct),t(t.t8, TestStruct), t(t.t9, TestStruct),t(t.t10, TestStruct),
            ps(ps.ns1, u8), ps(ps.ns2, u16), ps(ps.ns3, u32), ps(ps.ns4, u64), ps(ps.ns5, u128),
            ts(ts.ts6, TestStruct), ts(ts.ts7, TestStruct),ts(ts.ts8, TestStruct), ts(ts.ts9, TestStruct),ts(ts.ts10, TestStruct)
        )
    ));
}
*/