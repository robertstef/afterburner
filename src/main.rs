use instruction::decoder::*;
pub mod instruction;

fn main() {
    println!("Hello, world!");  
    let operation: DecodeOperation = DecodeOperation {
        mask: 123, 
        shift_amt: 1
    };
    let operations = vec![operation];
    decode(13321321, operations);
}
