use crate::instruction::decoder::decode;

pub struct ALUInstruction {
                         // Bit #s:
    condition: u8,       // 31-28
    is_immediate: bool,  // 25
    op_code: u8,         // 24-21
    set_cond: bool,      // 20
    op_register: u8,     // 19-16
    dst_register: u8,    // 15-12
    ss_op: u16           // 11-0

    // Note: bits 27-26 are used to verify if the
    //       instruction is an ALU instruction and 
    //       will be discarded when creating the struct.
}


impl ALUInstruction {
    
    pub fn new(instruction: u32) -> Self {
        let mut condition = 0;
        let mut verify = 0;
        let mut is_immediate = 0;
        let mut op_code = 0;
        let mut set_cond = 0;
        let mut op_register = 0;
        let mut dst_register = 0;
        let mut ss_op = 0;

        let mut fields: Vec<&mut u32> = vec![
            &mut condition, 
            &mut verify, 
            &mut is_immediate, 
            &mut op_code, 
            &mut set_cond, 
            &mut op_register, 
            &mut dst_register, 
            &mut ss_op
        ];

        let decode_ops: Vec<(u32, u32)> = vec![
            (0xF, 4),    // condition
            (0x3, 2),    // verify
            (0x1, 1),    // is_immediate
            (0xF, 4),    // op_code
            (0x1, 1),    // set_cond
            (0xF, 4),    // op_register
            (0xF, 4),    // dst_register
            (0xFFF, 12)  // ss_op
        ];

        decode(instruction, &mut fields, &decode_ops);

        if verify != 0x0 {
            panic!("Invalid ALU instruction. Bits 27 and 26 must be \
                    0x0 but were set to {:#02x}", verify)
        }
        
        ALUInstruction {
            condition: condition as u8,
            is_immediate: if is_immediate == 1 {true} else {false},
            op_code: op_code as u8,
            set_cond: if set_cond == 1 {true} else {false},
            op_register: op_register as u8,
            dst_register: dst_register as u8,
            ss_op: ss_op as u16
        }
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_invalid_alu_instruction() {
        let input_instruction: u32 = 0xFCFFFFFF;
        ALUInstruction::new(input_instruction);
    }

    #[test]
    fn test_create_new_alu_instruction() {
        let input_instruction: u32 = 0xC3AB2FB4;
        let alu_instruction: ALUInstruction = ALUInstruction::new(input_instruction);
        run_test(alu_instruction, 
            0xC, 
            true, 
            0xD, 
            false, 
            0xB, 
            0x2, 
            0xFB4);
    }

    fn run_test(instruction: ALUInstruction, 
                condition: u8, 
                is_immediate: bool, 
                op_code: u8, 
                set_cond: bool, 
                op_register: u8, 
                dst_register: u8, 
                ss_op: u16) {
        
        assert_eq!(instruction.condition, condition);
        assert_eq!(instruction.is_immediate, is_immediate);
        assert_eq!(instruction.op_code, op_code);
        assert_eq!(instruction.set_cond, set_cond);
        assert_eq!(instruction.op_register, op_register);
        assert_eq!(instruction.dst_register, dst_register);
        assert_eq!(instruction.ss_op, ss_op);
    }
}
