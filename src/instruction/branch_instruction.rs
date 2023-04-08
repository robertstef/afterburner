use crate::instruction::decoder::decode;

pub struct BranchInstruction {
                    // Bit #s:
    condition: u8,  // 31-28
    op_code: u8,    // 24
    offset: u32,    // 23-0
}

impl BranchInstruction {
    pub fn new(instruction: u32) -> Self {
        let mut condition: u32 = 0;
        let mut verify: u32 = 0;
        let mut op_code: u32 = 0;
        let mut offset: u32 = 0;

        let mut fields: Vec<&mut u32> = vec![
            &mut condition,
            &mut verify,
            &mut op_code,
            &mut offset
        ];

        let decode_ops: Vec<(u32, u32)> = vec![
            (0xF, 4),     // condition
            (0x5, 3),     // verify
            (0x1, 1),     // op_code
            (0xFFFF, 23)  // offset
        ];

        decode(instruction, &mut fields, &decode_ops);

        if verify != 0x5 {
            panic!("Invalid branch instruction. Bits 27-25 must be 0x5 
                    but were set to {:#02x}", verify)
        }

        BranchInstruction {
            condition: condition as u8,
            op_code: op_code as u8,
            offset: offeset
        }
    }
}