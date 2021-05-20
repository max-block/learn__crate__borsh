use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct A {
    x: u64,
    y: String,
}
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
enum CalcInstruction {
    Init,
    Plus { value: i32 },
    Minus { value: i32, comment: String },
}

fn main() {
    println!(
        "{:?}",
        A {
            x: 778,
            y: "bla".to_string()
        }
        .try_to_vec()
        .unwrap()
    );
    // [10, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 98, 108, 97]

    println!("{:?}", CalcInstruction::Init.try_to_vec().unwrap());
    // [0]

    println!("{:?}", CalcInstruction::Plus { value: 9 }.try_to_vec().unwrap());
    // [1, 9, 0, 0, 0]

    println!("{:?}", CalcInstruction::Minus { value: 9, comment: String::from("zzz") }.try_to_vec().unwrap());
    // [2, 9, 0, 0, 0, 3, 0, 0, 0, 122, 122, 122]

}
