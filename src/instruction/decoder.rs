pub struct DecodeOperation {
    pub mask: u32,
    pub shift_amt: u32,
}

pub fn decode(instruction: u32, operations: Vec<DecodeOperation>) -> Vec<u32> {

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
                shift_amt: 31
            }, 
            DecodeOperation {
                mask: 234, 
                shift_amt: 3
            }
        ];
        decode(1231321321, operations);
    }
}