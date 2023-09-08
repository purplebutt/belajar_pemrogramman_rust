#[allow(unused)]
use std:: {
    u8,
    u16, 
    u32,
    u64,
};


pub fn tipe_data_integer() {
    let unsign8 = 1_u8;
    let unsign16: u16 = 1;
    let unsign32 =  1u32;
    let unsign64 =  1_254_300_u64;


    println!("u8: {}", unsign8);
    println!("u16: {}", unsign16);
    println!("u32: {}", unsign32);
    println!("u64: {}", unsign64);
}

pub fn uint_min_max() {
    let ui8 = (u8::MIN, u8::MAX);
    let ui16 = (u16::MIN, u16::MAX);
    let ui32 = (u32::MIN, u32::MAX);
    let ui64 = (u64::MIN, u64::MAX);

    println!("{:?}", ui8);
    println!("{:?}", ui16);
    println!("{:?}", ui32);
    println!("{:?}", ui64);
}

pub fn int_min_max() {
    let int8 = (i8::MIN, i8::MAX);
    let int16 = (i16::MIN, i16::MAX);
    let int32 = (i32::MIN, i32::MAX);
    let int64 = (i64::MIN, i64::MAX);
    let int128 = (i128::MIN, i128::MAX);

    println!("{:?}", int8);
    println!("{:?}", int16);
    println!("{:?}", int32);
    println!("{:?}", int64);
    println!("{:?}", int128);
}

pub fn basic_math() {
    let n1 = 10;
    let n2 = 5;
    let n3 = n1+n2;
    let n4 = n1-n2;
    let n5 = n1*n2;
    let n6 = n1/n2;
    let n7 = n1%n2;

    println!("{} {} {} {} {} {} {}", n1, n2, n3, n4, n5, n6, n7)
}