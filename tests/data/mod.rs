/*
 * @file tampon/tests/datatest.rs
 *
 * @module tampon::tests
 *
 * @brief Contains struct of datas used for tests.
 * 
 * @details
 * Contains struct of datas used for tests.
 * 
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

use tampon::Tampon;

use crate::implementation::{ TestStruct } ;


 // Size of slices
static SLICESIZE: usize = 255;

// String constants
static STRING_0: &str = "I saw your text.";
static STRING_1: &str = "Écrits avec des charactères spéciaux tel que é ç à î ì ï.";
static STRING_2: &str = "";     // Empty string
static STRING_3: &str = "तुजो मजकूर पळयलो"; // Konkani
static STRING_4: &str = "Би таны бичвэрийг харсан"; // Mongolian
static STRING_5: &str = "Nobis tamquam probatus ad est, nam ex singulis volutpat. Pro et dicta dictas iriure, ius tibique patrioque ea. Quo meis aeque commune ea, ius ea case vocibus iracundia. Esse consul vis at, dolore dissentias vel ei. Qui tation civibus moderatius et, no mei suas harum conclusionemque. Eum et nonumy fastidii detracto, usu magna referrentur ea.";
static STRING_6: &str = "我看到了你的文字"; // Chinese simplified
static STRING_7: &str = "رأيت النص الخاص بك"; // Arabic
static STRING_8: &str = "私はあなたのテキストを見ました"; // Japanese
static STRING_9: &str = "Я бачив ваш текст"; // Ukrainian

pub struct DataNumerics {
    pub n1 :u8,
    pub n2: u16,
    pub n3 :u32,
    pub n4:u64,
    pub n5 :u128,
    pub n6 :f32,
    pub n7:f64,
    pub n8 :i8,
    pub n9:i16,
    pub n10 :i32
}

impl DataNumerics {
    pub fn new() -> DataNumerics{
        DataNumerics {
            n1: u8::MAX,
            n2: u16::MAX,
            n3: u32::MAX,
            n4: u64::MAX,
            n5: u128::MAX,
            n6: f32::MAX,
            n7: f64::MAX,
            n8: i8::MAX,
            n9: i16::MAX,
            n10: i32::MAX,
        }
    }

    pub fn get_size(&self, var_count : usize) -> usize {

        let mut size : usize = 0;

        if var_count >= 1 {
            size += core::mem::size_of::<u8>();
        }
        if var_count >= 2 {
            size += core::mem::size_of::<u16>();
        }
        if var_count >= 3 {
            size += core::mem::size_of::<u32>();
        }
        if var_count >= 4 {
            size += core::mem::size_of::<u64>();
        }
        if var_count >= 5 {
            size += core::mem::size_of::<u128>();
        }
        if var_count >= 6 {
            size += core::mem::size_of::<f32>();
        }
        if var_count >= 7 {
            size += core::mem::size_of::<f64>();
        }
        if var_count >= 8 {
            size += core::mem::size_of::<i8>();
        }
        if var_count >= 9 {
            size += core::mem::size_of::<i16>();
        }
        if var_count >= 10 {
            size += core::mem::size_of::<i32>();
        }

        size
    }

}

pub struct DataNumericsSlices {
    pub ns1: Vec<u8>,
    pub ns2: Vec<u16>,
    pub ns3: Vec<u32>,
    pub ns4: Vec<u64>,
    pub ns5: Vec<u128>,
    pub ns6: Vec<f32>,
    pub ns7: Vec<f64>,
    pub ns8: Vec<i8>,
    pub ns9: Vec<i16>,
    pub ns10: Vec<i32>
}

impl DataNumericsSlices {
    pub fn new() -> DataNumericsSlices{
        DataNumericsSlices {
            ns1: vec![u8::MAX;SLICESIZE],
            ns2: vec![u16::MAX;SLICESIZE],
            ns3: vec![u32::MAX;SLICESIZE],
            ns4: vec![u64::MAX;SLICESIZE],
            ns5: vec![u128::MAX;SLICESIZE],
            ns6: vec![f32::MAX;SLICESIZE],
            ns7: vec![f64::MAX;SLICESIZE],
            ns8: vec![i8::MAX;SLICESIZE],
            ns9: vec![i16::MAX;SLICESIZE],
            ns10: vec![i32::MIN;SLICESIZE]
        }
    }

    pub fn get_size(&self, var_count : usize) -> usize {

        let mut size : usize = 0;

        if var_count >= 1 {
            size += core::mem::size_of::<u8>() * self.ns1.len();
        }
        if var_count >= 2 {
            size += core::mem::size_of::<u16>() * self.ns2.len();
        }
        if var_count >= 3 {
            size += core::mem::size_of::<u32>() * self.ns3.len();
        }
        if var_count >= 4 {
            size += core::mem::size_of::<u64>() * self.ns4.len();
        }
        if var_count >= 5 {
            size += core::mem::size_of::<u128>() * self.ns5.len();
        }
        if var_count >= 6 {
            size += core::mem::size_of::<f32>() * self.ns6.len();
        }
        if var_count >= 7 {
            size += core::mem::size_of::<f64>() * self.ns7.len();
        }
        if var_count >= 8 {
            size += core::mem::size_of::<i8>() * self.ns8.len();
        }
        if var_count >= 9 {
            size += core::mem::size_of::<i16>() * self.ns9.len();
        }
        if var_count >= 10 {
            size += core::mem::size_of::<i32>() * self.ns10.len();
        }

        size
    }
}


pub struct DataStrings {
    pub s1: String,
    pub s2: String,
    pub s3: String,
    pub s4: String,
    pub s5: String,
    pub s6: String,
    pub s7: String,
    pub s8: String,
    pub s9: String,
    pub s10: String,
}


impl DataStrings {
    pub fn new() -> DataStrings{
        DataStrings {
            s1: String::from(STRING_0),
            s2: String::from(STRING_1),
            s3: String::from(STRING_2),
            s4: String::from(STRING_3),
            s5: String::from(STRING_4),
            s6: String::from(STRING_5),
            s7: String::from(STRING_6),
            s8: String::from(STRING_7),
            s9: String::from(STRING_8),
            s10: String::from(STRING_9),
        }
    }

    pub fn get_size(&self, var_count : usize) -> usize {

        let mut size : usize = 0;

        if var_count >= 1 {
            size += self.s1.len();
        }
        if var_count >= 2 {
            size += self.s2.len();
        }
        if var_count >= 3 {
            size += self.s3.len();
        }
        if var_count >= 4 {
            size += self.s4.len();
        }
        if var_count >= 5 {
            size += self.s5.len();
        }
        if var_count >= 6 {
            size += self.s6.len();
        }
        if var_count >= 7 {
            size += self.s7.len();
        }
        if var_count >= 8 {
            size += self.s8.len();
        }
        if var_count >= 9 {
            size += self.s9.len();
        }
        if var_count >= 10 {
            size += self.s10.len();
        }

        size
    }

}



pub struct DataStringsSlices {
    pub ss1: Vec<String>,
    pub ss2: Vec<String>,
    pub ss3: Vec<String>,
    pub ss4: Vec<String>,
    pub ss5: Vec<String>,
    pub ss6: Vec<String>,
    pub ss7: Vec<String>,
    pub ss8: Vec<String>,
    pub ss9: Vec<String>,
    pub ss10: Vec<String>,
}


impl DataStringsSlices {
    pub fn new() -> DataStringsSlices{

        let mut ss1 = Vec::new();
        ss1.push(String::from(STRING_0));

        let mut ss2 = Vec::new();
        ss2.push(String::from(STRING_0));
        ss2.push(String::from(STRING_1));

        let mut ss3 = Vec::new();
        ss3.push(String::from(STRING_0));
        ss3.push(String::from(STRING_1));
        ss3.push(String::from(STRING_2));

        let ss4 = Vec::new();

        let mut ss5 = Vec::new();
        ss5.push(String::from(STRING_0));
        ss5.push(String::from(STRING_1));
        ss5.push(String::from(STRING_2));
        ss5.push(String::from(STRING_3));
        ss5.push(String::from(STRING_4));

        let mut ss6 = Vec::new();
        ss6.push(String::from(STRING_0));
        ss6.push(String::from(STRING_1));
        ss6.push(String::from(STRING_2));
        ss6.push(String::from(STRING_3));
        ss6.push(String::from(STRING_4));
        ss6.push(String::from(STRING_5));

        let mut ss7 = Vec::new();
        ss7.push(String::from(STRING_0));
        ss7.push(String::from(STRING_1));
        ss7.push(String::from(STRING_2));
        ss7.push(String::from(STRING_3));
        ss7.push(String::from(STRING_4));
        ss7.push(String::from(STRING_5));
        ss7.push(String::from(STRING_6));
        ss7.push(String::from(STRING_7));

        let mut ss8 = Vec::new();
        ss8.push(String::from(STRING_0));
        ss8.push(String::from(STRING_1));
        ss8.push(String::from(STRING_2));
        ss8.push(String::from(STRING_3));
        ss8.push(String::from(STRING_4));
        ss8.push(String::from(STRING_5));
        ss8.push(String::from(STRING_6));
        ss8.push(String::from(STRING_7));
        ss8.push(String::from(STRING_8));

        let mut ss9 = Vec::new();
        ss9.push(String::from(STRING_0));
        ss9.push(String::from(STRING_1));
        ss9.push(String::from(STRING_2));
        ss9.push(String::from(STRING_3));
        ss9.push(String::from(STRING_4));
        ss9.push(String::from(STRING_5));
        ss9.push(String::from(STRING_6));
        ss9.push(String::from(STRING_7));
        ss9.push(String::from(STRING_8));
        ss9.push(String::from(STRING_9));

        let mut ss10 = Vec::new();
        ss10.push(String::from(STRING_0));
        ss10.push(String::from(STRING_1));
        ss10.push(String::from(STRING_2));
        ss10.push(String::from(STRING_3));
        ss10.push(String::from(STRING_4));
        ss10.push(String::from(STRING_5));
        ss10.push(String::from(STRING_6));
        ss10.push(String::from(STRING_7));
        ss10.push(String::from(STRING_8));
        ss10.push(String::from(STRING_9));
        ss10.push(String::from(STRING_0));
        ss10.push(String::from(STRING_1));
        ss10.push(String::from(STRING_2));
        ss10.push(String::from(STRING_3));
        ss10.push(String::from(STRING_4));
        ss10.push(String::from(STRING_5));
        ss10.push(String::from(STRING_6));
        ss10.push(String::from(STRING_7));
        ss10.push(String::from(STRING_8));
        ss10.push(String::from(STRING_9));




        DataStringsSlices {

            ss1,ss2,ss3,ss4,ss5,ss6,ss7,ss8,ss9,ss10
        }
    }

    pub fn get_size(&self, var_count : usize) -> usize {

        let mut size : usize = 0;

        if var_count >= 1 {
            for elem in self.ss1.iter() {
                size += elem.len();
            }
        }
        if var_count >= 2 {
            for elem in self.ss2.iter() {
                size += elem.len();
            }
        }
        if var_count >= 3 {
            for elem in self.ss3.iter() {
                size += elem.len();
            }
        }
        if var_count >= 4 {
            for elem in self.ss4.iter() {
                size += elem.len();
            }
        }
        if var_count >= 5 {
            for elem in self.ss5.iter() {
                size += elem.len();
            }
        }
        if var_count >= 6 {
            for elem in self.ss6.iter() {
                size += elem.len();
            }
        }
        if var_count >= 7 {
            for elem in self.ss7.iter() {
                size += elem.len();
            }
        }
        if var_count >= 8 {
            for elem in self.ss8.iter() {
                size += elem.len();
            }
        }
        if var_count >= 9 {
            for elem in self.ss9.iter() {
                size += elem.len();
            }
        }
        if var_count >= 10 {
            for elem in self.ss10.iter() {
                size += elem.len();
            }
        }

        size
    }

}

pub struct DataTampons {
    pub t1: TestStruct,
    pub t2: TestStruct,
    pub t3: TestStruct,
    pub t4: TestStruct,
    pub t5: TestStruct,
    pub t6: TestStruct,
    pub t7: TestStruct,
    pub t8: TestStruct,
    pub t9: TestStruct,
    pub t10: TestStruct,
}

impl DataTampons {
    pub fn new() -> DataTampons{
        DataTampons {
            t1: TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 10),
            t2: TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 20),
            t3: TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 30),
            t4: TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 40),
            t5: TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 50),
            t6: TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 60),
            t7: TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 70),
            t8: TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 80),
            t9: TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 90),
            t10: TestStruct::new(u8::MAX, u32::MAX, f64::MAX, 100)
        }
    }

    pub fn get_size(&self, var_count : usize) -> usize {

        let mut size : usize = 0;

        if var_count >= 1 {
            size += self.t1.bytes_size();
        }
        if var_count >= 2 {
            size += self.t2.bytes_size();
        }
        if var_count >= 3 {
            size += self.t3.bytes_size();
        }
        if var_count >= 4 {
            size += self.t4.bytes_size();
        }
        if var_count >= 5 {
            size += self.t5.bytes_size();
        }
        if var_count >= 6 {
            size += self.t6.bytes_size();
        }
        if var_count >= 7 {
            size += self.t7.bytes_size();
        }
        if var_count >= 8 {
            size += self.t8.bytes_size();
        }
        if var_count >= 9 {
            size += self.t9.bytes_size();
        }
        if var_count >= 10 {
            size += self.t10.bytes_size();
        }

        size
    }
}

pub struct DataTamponsSlices{
    pub ts1:Vec<TestStruct>,
    pub ts2:Vec<TestStruct>,
    pub ts3:Vec<TestStruct>,
    pub ts4:Vec<TestStruct>,
    pub ts5:Vec<TestStruct>,
    pub ts6:Vec<TestStruct>,
    pub ts7:Vec<TestStruct>,
    pub ts8:Vec<TestStruct>,
    pub ts9:Vec<TestStruct>,
    pub ts10:Vec<TestStruct>,
}

impl DataTamponsSlices {
    pub fn new() -> DataTamponsSlices{
        DataTamponsSlices {
            ts1: vec_of_test_struct(10),
            ts2: vec_of_test_struct(20),
            ts3: vec_of_test_struct(30),
            ts4: vec_of_test_struct(40),
            ts5: vec_of_test_struct(50),
            ts6: vec_of_test_struct(60),
            ts7: vec_of_test_struct(70),
            ts8: vec_of_test_struct(80),
            ts9: vec_of_test_struct(90),
            ts10: vec_of_test_struct(100),
        }
    }


    pub fn get_size(&self, var_count : usize) -> usize {

        let mut size : usize = 0;

        if var_count >= 1 {
            size += self.ts1[0].bytes_size() * self.ts1.len()
        }
        if var_count >= 2 {
            size += self.ts2[0].bytes_size() * self.ts2.len()
        }
        if var_count >= 3 {
            size += self.ts3[0].bytes_size() * self.ts3.len()
        }
        if var_count >= 4 {
            size += self.ts4[0].bytes_size() * self.ts4.len()
        }
        if var_count >= 5 {
            size += self.ts5[0].bytes_size() * self.ts5.len()
        }
        if var_count >= 6 {
            size += self.ts6[0].bytes_size() * self.ts6.len()
        }
        if var_count >= 7 {
            size += self.ts7[0].bytes_size() * self.ts7.len()
        }
        if var_count >= 8 {
            size += self.ts8[0].bytes_size() * self.ts8.len()
        }
        if var_count >= 9 {
            size += self.ts9[0].bytes_size() * self.ts9.len()
        }
        if var_count >= 10 {
            size += self.ts10[0].bytes_size() * self.ts10.len()
        }

        size
    }
}

// Print macro test result and assert.
pub fn macro_test_validation(expected:usize, result:usize) -> bool {

    println!("Expected={}, Result={}, Diff={}",result, expected, if expected > result {
        expected - result
    } else {
        result - expected
    });

    expected == result
}


// Create a vector of TestStruct
pub fn vec_of_test_struct(size:usize) -> Vec<TestStruct>{

    let mut vects: Vec<TestStruct> = Vec::new();

    for i in 0..size {
        vects.push(TestStruct::new(i as u8, i as u32, i as f64, i * 10 + 1));
    }

    vects
}