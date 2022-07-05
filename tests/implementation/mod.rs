/*
 * @file tampon/tests/tampon.rs
 *
 * @module tampon::tests
 *
 * @brief Contains implementation of Tampon trait used for tests.
 * 
 * @details
 * Contains implementation of Tampon trait used for tests.
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

use std::vec;

use tampon::{Tampon, bytes_size, to_buffer};

// Struct used to test Tampon traits in macros
 pub struct TestStruct {
    _f1:u8,
    _f2:u32,
    _f3:f64,
    f4:InnerStruct,
    v1:Vec<u8>,
    v2:Vec<f64>,
    v3:Vec<InnerStruct>
 }

 impl TestStruct {
    pub fn new(f1:u8, f2:u32, f3:f64, vsize:usize) -> TestStruct {

        let f4 = InnerStruct::new(0,1234567892);

        let v1:Vec<u8> = vec![f1;vsize]; 
        let v2:Vec<f64> = vec![f3;vsize];

        let mut v3:Vec<InnerStruct> = Vec::new();

        for i in 0..f1 {
            v3.push(InnerStruct::new(i, (i as i128 * i as i128) as i128));
        }

        TestStruct {
            _f1: f1,_f2: f2,_f3: f3,f4,v1,v2,v3
        }
    }
 }


impl Tampon for TestStruct {
    fn bytes_size(&self) -> usize {
        bytes_size!(n(self._f1,u8),n(self._f2,u32),n(self._f3,f64),t(self.f4,InnerStruct),ns(self.v1, u8), 
        ns(self.v2,f64), ts(self.v3,InnerStruct))
    }

    fn to_buffer(&self, buffer : &mut [u8]) -> usize {

0
        //to_buffer!(buffer, 0, 
        //    p(self._f1,u8))

        /*
        let r:usize = to_buffer!(&mut buffer, 0, 
            p(self._f1,u8),
            p(self._f2,u32),
            p(self._f3,f64),
            ps(self.v1, u8),
            ps(self.v2,f64));

            r
*/
        /*
        to_buffer!(&mut buffer, 0, 
            p(self._f1,u8),
            p(self._f2,u32),
            p(self._f3,f64),
            t(self.f4,InnerStruct),
            ps(self.v1, u8),
            ps(self.v2,f64), 
            ts(self.v3,InnerStruct))
            */


    }

    fn from_buffer(&mut self, buffer : &[u8]) -> usize {
        //TODO
        0
    }
}


 // Struct used as inner struct of test struct
 struct InnerStruct {
    _f1:u8,
    _f2:i128
 }

 impl InnerStruct {
    pub fn new(f1:u8, f2:i128) -> InnerStruct {
        InnerStruct {
            _f1: f1,_f2: f2
        }
    }
 }

 impl Tampon for InnerStruct {
    fn bytes_size(&self) -> usize {
        bytes_size!(n(self._f1,u8),n(self._f2,i128))
    }

    fn to_buffer(&self, buffer : &mut [u8]) -> usize {
        //to_buffer!(buffer,0,p(self._f1,u8),p(self._f2,i128))
        0
    }

    fn from_buffer(&mut self, buffer : &[u8]) -> usize {
        //TODO
        0
    }
}