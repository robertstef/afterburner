pub struct Cpu {
  // Link Register stores the return address to come to after a subroutine call using the BL instruction
  pub r14_link_register: u23,
  pub r15_program_counter: u32,
  pub current_program_status_register: u32,
  /*  
  Register 13 is sometimes a stack pointer 
    if the CPU is in THUMB mode, r13 is a stack pointer
    if the CPU is in ARM mode, the user can use r13 as a stack pointer OR as a GPR 
  */
  pub general_purpose_registers: [u32; 13]
}

impl Cpu {
  pub fn new() -> self {
    Cpu {
      r14_link_register: 0,
      r15_program_counter: 0, 
      general_purpose_registers: [0; 13]
    }
  }

  pub fn doDataOp(op: Instruction, op1: u32, op2: u32, ) {
    match op {
      AND | TST => {
        op1 & op2
      }
      EOR | TEQ => {
        op1 ^ op2
      }
      ORR => {
        op1 | op2
      }
      MOV => {
        op2
      }
      MVN => {
        !op2
      }
      BIC => {
        op1 & !(op2)
      }
      _ => todo!()
    }
  }

  // Dunno what a good interface to this class would be really 
  pub fn step(&mut self, instuction: u32){
    let instructionType = self.determineType(instruction);
  }
}