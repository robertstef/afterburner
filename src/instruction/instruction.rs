trait Instruction {
    fn create(&self) -> String;
    fn another(&self) -> String;
}

pub struct ALUInstruction {
    pub condition: u8, // 4 bit number: bits 31-28
    // Need to verify bits 27 and 26 are 0 somehow?
    pub is_immediate: bool, // 1 bit: bit 25
    pub op_code: u32,  // 4 bits: bits 24-21
    pub set_cond_code: bool, // 1 bit: bit 20
    pub first_op_register: u8, // 4 bit: 19-16
    pub dest_register: u8, // 4 bit: 15-12

    // Handles shift etc. will need to mask on the fly to see what's happening
    pub shift_and_second_op: u16 // 11-0
}



impl ALUInstruction {
    fn new(instruction: u32) -> Self {
        //                   31   27   23   19   15   11    7    3
        let COND: u32 =     0b1111_0000_0000_0000_0000_0000_0000_0000;
        let VERIFY: u32 =   0b0000_1100_0000_0000_0000_0000_0000_0000;
        let IMM: u32 =      0b0000_0010_0000_0000_0000_0000_0000_0000;
        let OP: u32 =       0b0000_0001_1110_0000_0000_0000_0000_0000;
        let SET_COND: u32 = 0b0000_0000_0001_0000_0000_0000_0000_0000;
        let OP_REG: u32 =   0b0000_0000_0000_1111_0000_0000_0000_0000;
        let DST_REG: u32 =  0b0000_0000_0000_0000_1111_0000_0000_0000;
        let SS_OP: u32 =    0b0000_0000_0000_0000_0000_1111_1111_1111;

        Self{
            condition: 0, 
            is_immediate: true, 
            op_code: 32}
    }
}

impl Instruction for ALUInstruction {
    fn create(&self) -> String {
        return String::from("hello world");
    }
    fn another(&self) -> String {
        return String::from("another");
    }
}