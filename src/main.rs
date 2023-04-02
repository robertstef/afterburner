use instruction::decoder::*;

use crate::instruction::alu_instruction::ALUInstruction;
pub mod instruction;

fn main() {
    println!("Hello, world!");  
    let instruction = ALUInstruction::new(0b0000_0000_0000_0000_0001_0000_0000_0010);
    println!("{:?}", instruction);
}
