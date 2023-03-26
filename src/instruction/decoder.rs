use crate::instruction::instruction::ALUInstruction;
pub struct DecodeOperation {
    pub mask: u32,
    pub shift_amt: u32,
}
pub fn decode_alu_instruction(instruction: u32, operations: &Vec<DecodeOperation>) -> ALUInstruction {
    let decoded_fields = decode(instruction, operations); 
    ALUInstruction { 
        condition: (decoded_fields[0] as u8), 
        is_immediate: (decoded_fields[1] != 0), 
        op_code: (decoded_fields[2] as u8), 
        set_cond_code: (decoded_fields[3] != 0), 
        first_op_register: (decoded_fields[4] as u8), 
        dest_register: (decoded_fields[5] as u8), 
        shift_and_second_op: (decoded_fields[6] as u16) 
    }
}

fn decode(instruction: u32, operations: &Vec<DecodeOperation>) -> Vec<u32> {

    if _check_shift_amt_correct(&operations) == false {
        panic!("Shift amount must add to 32. Check your shift amounts.")
    }
    
    let mut instruction_fields: Vec<u32> = Vec::new();
    for op in operations {
        let field = instruction & op.mask;
        instruction_fields.push(field);
        instruction >> op.shift_amt;
    }
    instruction_fields
}

fn _check_shift_amt_correct(operations: &Vec<DecodeOperation>) -> bool {
    let SHIFT_AMT = 32;
    let mut tot = 0;
    for op in operations {
        tot += op.shift_amt
    }
    
    tot == SHIFT_AMT
}



#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_decode_panic() {
        let operations = vec![
            DecodeOperation {
                mask: 123, 
                shift_amt: 32
            }, 
            DecodeOperation {
                mask: 234, 
                shift_amt: 3
            }
        ];
        decode(1231321321, &operations);
    }

    #[test]
    fn test_ok() {
        let operations = vec![
            DecodeOperation {
                mask: 0b0000_0000_0000_0000_0000_1111_1111_1111, 
                shift_amt: 12
            }, 
            DecodeOperation {
                mask: 0b0000_0000_0000_0000_1111_0000_0000_0000, 
                shift_amt: 4
            },
            DecodeOperation {
                mask: 0b0000_0000_0000_1111_0000_0000_0000_0000, 
                shift_amt: 4
            },
            DecodeOperation {
                mask: 0b0000_0000_0001_0000_0000_0000_0000_0000, 
                shift_amt: 1
            }, 
            DecodeOperation {
                mask: 0b0000_0001_1110_0000_0000_0000_0000_0000, 
                shift_amt: 4
            },
            DecodeOperation {
                mask: 0b0000_0010_0000_0000_0000_0000_0000_0000, 
                shift_amt: 1
            },
            DecodeOperation {
                mask: 0b0000_1100_0000_0000_0000_0000_0000_0000, 
                shift_amt: 2
            },
            DecodeOperation {
                mask: 0b1111_0000_0000_0000_0000_0000_0000_0000, 
                shift_amt: 4
            }
        ];
        // a semi real instruction 
        let result = decode(0b0000_0000_0000_0000_0001_0000_0000_0010, &operations);
        assert_eq!(result.len(), operations.len()) 
    }
}