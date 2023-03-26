extern crate num;
#[macro_use]
extern crate num_derive;

#[derive(FromPrimitive)]
pub enum OPCODE {
  AND = 0, 
  EOR, 
  SUB, 
  RSB, 
  ADD, 
  ADC, 
  SBC, 
  RSC, 
  TST, 
  CMP, 
  CMN, 
  ORR, 
  MOV, 
  BIC, 
  MVN
}

