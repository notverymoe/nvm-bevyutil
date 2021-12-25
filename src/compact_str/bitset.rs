/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

const VALUE_MASKS: [u8; 8] = [
    0b0000_0001, 0b0000_0011, 0b0000_0111, 0b0000_1111,
    0b0001_1111, 0b0011_1111, 0b0001_1111, 0b1111_1111, 
];

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BitSet<const N: usize>(pub [u8; N]);

impl<const N: usize> Default for BitSet<N> {
    fn default() -> Self {
        Self::empty()
    }
}

macro_rules! write_bits {
    ($dst:ident, $offset:ident, $bit_width:ident, $value:ident) => {
        let byte_from       = $offset/8;
        let byte_bit_offset = $offset - byte_from*8;

        if byte_bit_offset + $bit_width <= 8 {
            $dst.0[byte_from] = do_encode(&$dst.0, byte_from, byte_bit_offset, $bit_width, $value);
        } else {
            let value_bits_first  = 8 - byte_bit_offset; 
            let value_bits_second = $bit_width - value_bits_first; 
    
            $dst.0[byte_from  ] = do_encode(&$dst.0, byte_from,   byte_bit_offset, value_bits_first,  $value >> value_bits_second);
            $dst.0[byte_from+1] = do_encode(&$dst.0, byte_from+1,               0, value_bits_second, $value);
        }
    };
}

impl<const N: usize> BitSet<N> {

    pub const fn empty() -> Self {
        Self([0; N])
    }

    pub const fn from_buffer(buffer: &[u8], max_bytes: usize, bit_width: usize) -> Self {
        debug_assert!(bit_width > 0 && bit_width < 8, "value_bits should be between 1-8");

        let mut result = Self::empty();

        let mut offset = 0;
        let mut byte   = 0;
        while offset < N*8 && byte < max_bytes {
            let value = buffer[byte];

            write_bits!(result, offset, bit_width, value);

            offset += bit_width;
            byte   += 1;
        }

        result
    }

    pub fn set_bits(&mut self, offset: usize, bit_width: usize, value: u8) {
        debug_assert!(bit_width > 0 && bit_width < 8, "value_bits should be between 1-8");
        write_bits!(self, offset, bit_width, value);
    }

    pub const fn get_bits(&self, offset: usize, count: usize) -> u8 {
        debug_assert!(count > 0 && count < 8, "value_bits should be between 1-8");
    
        let byte_from = offset/8;
        let byte_bit_offset = offset - byte_from*8;
    
        if byte_bit_offset + count <= 8 {
            do_decode(&self.0, byte_from, byte_bit_offset, count)
        } else {
            let value_bits_first  =      8 - byte_bit_offset; 
            let value_bits_second = count - value_bits_first; 

            let msb = do_decode(&self.0, byte_from, byte_bit_offset, value_bits_first) << value_bits_second;
            let lsb = if byte_from+1 < N { do_decode(&self.0, byte_from+1, 0, value_bits_second) } else { 0 };

            msb | lsb
        }
    }
}

const fn do_encode(arr: &[u8], idx: usize, offset: usize, count: usize, value: u8) -> u8 {
    let mask  = !(VALUE_MASKS[count - 1]         << ((8 - offset) - count));
    let value = (value & VALUE_MASKS[count - 1]) << ((8 - offset) - count);
    (arr[idx] & mask) | value
}

const fn do_decode(arr: &[u8], idx: usize, offset: usize, count: usize) -> u8 {
    (arr[idx] >> ((8 - offset) - count)) & VALUE_MASKS[count - 1]
}