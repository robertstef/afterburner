pub fn decode(instruction: u32, 
              fields: &mut Vec<&mut u32>, 
              decode_ops: &Vec<(u32, u32)>) {
    
    const SHIFT_AMT: u32 = 32;
    let mut total_shift_amt = 0;
    for (_, shift_amt) in decode_ops {
        total_shift_amt += shift_amt;
    }

    if total_shift_amt != SHIFT_AMT { 
        panic!("Shift amount must add to 32 but instead added to {total_shift_amt}")
    }

    if fields.len() != decode_ops.len() {
        panic!("Number of fields and decode operations is not equal. There must be one field \
                for each decode operation.")
    }

    let mut mut_instruction = instruction;
    for i in (0..fields.len()).rev() {
        let (op, shift_amt) = decode_ops[i];
        *fields[i] = mut_instruction & op;
        mut_instruction = mut_instruction >> shift_amt;
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_shift_amt_incorrect() {
        let instruction: u32 = 0xF0000000;
        let mut f1: u32 = 1;
        let mut f2: u32 = 2;
        let mut fields: Vec<&mut u32> = vec![&mut f1, &mut f2];
        let decode_ops: Vec<(u32, u32)> = vec![(1, 1), (2, 2)];
        decode(instruction, &mut fields, &decode_ops);
    }

    #[test]
    #[should_panic]
    fn test_fields_decode_ops_not_equal() {
        let instruction: u32 = 0xF0000000;
        let mut f1: u32 = 1;
        let mut f2: u32 = 2;
        let mut f3: u32 = 3;
        let mut fields: Vec<&mut u32> = vec![&mut f1, &mut f2, &mut f3];
        let decode_ops: Vec<(u32, u32)> = vec![(1, 16), (2, 16)];
        decode(instruction, &mut fields, &decode_ops);
    }

    #[test]
    fn test_decode_instruction() {
        // Test instruction architecture:
        // | 4 bits | 8 bits | 8 bits | 12 bits |

        let instruction: u32 = 0xC21BDA79;
        let mut f1_actual: u32 = 0;
        let mut f2_actual: u32 = 0;
        let mut f3_actual: u32 = 0;
        let mut f4_actual: u32 = 0;

        let f1_expected: u32 = 0xC;
        let f2_expected: u32 = 0x21;
        let f3_expected: u32 = 0xBD;
        let f4_expected: u32 = 0xA79;

        let mut fields: Vec<&mut u32> = vec![&mut f1_actual, &mut f2_actual, &mut f3_actual, &mut f4_actual];
        let decode_ops: Vec<(u32, u32)> = vec![
            (0xF, 4),
            (0xFF, 8),
            (0xFF, 8),
            (0xFFF, 12)
        ];

        decode(instruction, &mut fields, &decode_ops);
        
        assert_eq!(f1_actual, f1_expected);
        assert_eq!(f2_actual, f2_expected);
        assert_eq!(f3_actual, f3_expected);
        assert_eq!(f4_actual, f4_expected);

    }

}