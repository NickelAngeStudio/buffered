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

use tampon::deserialize;
pub use tampon::{Tampon, bytes_size, serialize};

use crate::data::{do_vecs_match, do_vecs_eq_match};

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


impl Tampon<TamponS1> for TamponS1 {
    fn bytes_size(&self) -> usize {
        bytes_size!((self._f1):u8, (self._f2):u32, (self._f3):f64, (&self.f4):TamponS2, [self.v1]:u8, [self.v2]:f64, [self.v3]:TamponS2)
    }

    fn serialize(&self, buffer : &mut [u8]) -> usize {
        serialize!(buffer, to_size, (self._f1):u8, (self._f2):u32, (self._f3):f64, (&self.f4):TamponS2, [&self.v1]:u8, [&self.v2]:f64, [&self.v3]:TamponS2);
        to_size
    }

    fn deserialize(buffer : &[u8]) -> (TamponS1, usize) {
        
        deserialize!(buffer, from_size, (_f1):u8, (_f2):u32, (_f3):f64, (f4):TamponS2, [v1]:u8, [v2]:f64, [v3]:TamponS2);

        (TamponS1 {
            _f1,_f2,_f3,f4,v1,v2,v3
        }, from_size)

    }
}

impl PartialEq for TamponS1 {
    fn eq(&self, other: &Self) -> bool {
        self._f1 == other._f1 && self._f2 == other._f2 && self._f3 == other._f3 && self.f4.eq(&other.f4)
        && do_vecs_match(&self.v1,&other.v1) && do_vecs_match(&self.v2,&other.v2) && do_vecs_eq_match(&self.v3,&other.v3)
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

 impl Tampon<TamponS2> for TamponS2 {
    fn bytes_size(&self) -> usize {
        bytes_size!((self._f1):u8, (self._f2):i128)
    }

    fn serialize(&self, buffer : &mut [u8]) -> usize {
        serialize!(buffer, to_size, (self._f1):u8, (self._f2):i128);
        to_size
    }

    fn deserialize(buffer : &[u8]) -> (TamponS2, usize) {
        deserialize!(buffer, from_size, (_f1):u8, (_f2):i128);

        (TamponS2 {
            _f1,_f2
        }, from_size)
    }    
}

impl PartialEq for TamponS2 {
    fn eq(&self, other: &Self) -> bool {
        self._f1 == other._f1 && self._f2 == other._f2
    }
}