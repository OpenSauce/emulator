#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flags_register_to_u8() {
        let flags = FlagsRegister {
            zero: true,
            subtract: false,
            half_carry: true,
            carry: false,
        };
        let byte: u8 = flags.into();
        assert_eq!(byte, 0b10100000); // Expected binary representation
    }

    #[test]
    fn test_u8_to_flags_register() {
        let byte: u8 = 0b10100000;
        let flags = FlagsRegister::from(byte);

        assert_eq!(
            flags,
            FlagsRegister {
                zero: true,
                subtract: false,
                half_carry: true,
                carry: false,
            }
        );
    }

    #[test]
    fn test_round_trip_conversion() {
        let original_flags = FlagsRegister {
            zero: true,
            subtract: true,
            half_carry: false,
            carry: true,
        };
        let byte: u8 = original_flags.into();
        let converted_flags = FlagsRegister::from(byte);

        assert_eq!(original_flags, converted_flags);
    }
}
