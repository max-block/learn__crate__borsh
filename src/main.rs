use std::vec;

use borsh::{BorshDeserialize, BorshSerialize, try_to_vec_with_schema};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct S1 {
    x: u64,
    y: String,
}

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct S2 {
    x: u16,
    y: Vec<String>,
}
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
enum E1 {
    Init,
    Plus { value: i32 },
    Minus { value: i32, comment: String },
    Divide { value: i32, comment: String },
    Reset,
}

fn structs() {
    println!("{:?}", S1 { x: 778, y: String::from("bla") }.try_to_vec().unwrap());
    // [10, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 98, 108, 97]

    println!("{:?}", S2 { x: 777, y: vec![String::from("one"), String::from("two")] }.try_to_vec().unwrap());
    // [9, 3, 2, 0, 0, 0, 3, 0, 0, 0, 111, 110, 101, 3, 0, 0, 0, 116, 119, 111]
}

fn enums() {
    println!("{:?}", E1::Init.try_to_vec().unwrap());
    // [0]

    println!("{:?}", E1::Plus { value: 9 }.try_to_vec().unwrap());
    // [1, 9, 0, 0, 0]

    println!("{:?}", E1::Minus { value: 9, comment: String::from("zzz") }.try_to_vec().unwrap());
    // [2, 9, 0, 0, 0, 3, 0, 0, 0, 122, 122, 122]

    println!("{:?}", E1::Divide { value: 9, comment: String::from("zzz") }.try_to_vec().unwrap());
    // [3, 9, 0, 0, 0, 3, 0, 0, 0, 122, 122, 122]

    println!("{:?}", E1::Reset.try_to_vec().unwrap());
    // [4]
}

fn primitives() {
    println!("{:?}", try_to_vec_with_schema(&777u64).unwrap());
    // it looks wierd :(
}

fn main() {
    structs();
    enums();
    primitives();
}
