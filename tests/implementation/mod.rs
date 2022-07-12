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

pub use tampon::{Tampon, bytes_size, to_buffer};

// Struct used to test Tampon traits in macros
 pub struct TamponS1 {
    _f1:u8,
    _f2:u32,
    _f3:f64,
    f4:TamponS2,
    v1:Vec<u8>,
    v2:Vec<f64>,
    v3:Vec<TamponS2>
 }

 impl TamponS1 {
    pub fn new(f1:u8, f2:u32, f3:f64, vsize:usize) -> TamponS1 {

        let f4 = TamponS2::new(0,1234567892);

        let v1:Vec<u8> = vec![f1;vsize]; 
        let v2:Vec<f64> = vec![f3;vsize];

        let mut v3:Vec<TamponS2> = Vec::new();

        for i in 0..f1 {
            v3.push(TamponS2::new(i, (i as i128 * i as i128) as i128));
        }

        TamponS1 {
            _f1: f1,_f2: f2,_f3: f3,f4,v1,v2,v3
        }
    }
 }


impl Tampon for TamponS1 {
    fn bytes_size(&self) -> usize {
        bytes_size!((self._f1):u8, (self._f2):u32, (self._f3):f64, (&self.f4):InnerStruct, [self.v1]:u8, [self.v2]:f64, [self.v3]:InnerStruct)
    }

    fn to_buffer(&self, buffer : &mut [u8]) -> usize {
        to_buffer!(buffer, 0, N(self._f1, self._f2, self._f3), T(self.f4), N[self.v1, self.v2], T[self.v3])
    }

    fn from_buffer(&mut self, buffer : &[u8]) -> usize {
        //TODO
        0
    }
}


 // Struct used as inner struct of test struct
 pub struct TamponS2 {
    _f1:u8,
    _f2:i128
 }

 impl TamponS2 {
    pub fn new(f1:u8, f2:i128) -> TamponS2 {
        TamponS2 {
            _f1: f1,_f2: f2
        }
    }
 }

 impl Tampon for TamponS2 {
    fn bytes_size(&self) -> usize {
        bytes_size!((self._f1):u8, (self._f2):i128)
    }

    fn to_buffer(&self, buffer : &mut [u8]) -> usize {
        to_buffer!(buffer, 0, N(self._f1, self._f2))
    }

    fn from_buffer(&mut self, buffer : &[u8]) -> usize {
        //TODO
        0
    }
}