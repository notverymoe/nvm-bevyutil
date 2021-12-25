/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

#[cfg(test)] mod test;
mod bitset;

use std::{fmt::Debug, str::FromStr};

use bitset::*;

pub type CompactStr16  = CompactStr< 2>;
pub type CompactStr32  = CompactStr< 4>;
pub type CompactStr48  = CompactStr< 6>;
pub type CompactStr64  = CompactStr< 8>;
pub type CompactStr96  = CompactStr<12>;
pub type CompactStr128 = CompactStr<16>;
pub type CompactStr192 = CompactStr<24>;
pub type CompactStr256 = CompactStr<32>;

#[repr(transparent)]
#[derive(Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CompactStr<const WIDTH: usize>(BitSet<WIDTH>);

impl<const WIDTH: usize> FromStr for CompactStr<WIDTH> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_parse_str(s)
    }
}

impl<const WIDTH: usize> Debug for CompactStr<WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_str())
    }
}

impl<const WIDTH: usize> CompactStr<WIDTH> {

    pub const fn empty() -> Self {
        Self(BitSet::empty())
    }

    pub const fn try_parse_str(s: &str) -> Result<Self, &'static str> {
        if s.len() <= WIDTH*8/5 {
            Ok(Self::parse_str(s))
        } else {
            Err("String cannot fit into CompactStr")
        }
    }

    pub const fn parse_str(s: &str) -> Self {
        Self::from_ascii_bytes(s.as_bytes())
    }

    pub const fn from_ascii_bytes(chars: &[u8]) -> Self {
        let mut buffer = [0; 64];

        let max_len = const_min(chars.len(), WIDTH*8/5);
        let mut i = 0;
        while i < max_len {
            let character = chars[i];

            buffer[i] = match character {
                65..=90  => character-63, /* A-Z -> 2-27 */ 
                97..=122 => character-95, /* a-z -> 2-27*/ 
                _        => 1             /* Spc */
            };

            i += 1;
        }

        Self(BitSet::from_buffer(&buffer, max_len, 5))
    }
}

impl<const WIDTH: usize> CompactStr<WIDTH> {
    pub fn to_str(&self) -> String {
        self.chars().collect()   
    }

    pub fn set_ascii(&mut self, idx: usize, value: u8) {
        self.0.set_bits(idx*5, 5, value)
    }

    pub fn at<T: From<u8>>(&self, index: usize) -> T {
        if index >= Self::capacity() { panic!("Attempt to index out of bounds!") }
        self.at_unchecked::<T>(index)
    }

    pub fn len_bytes(&self) -> usize {
        (self.len()*5 + 7 /* round up */)/8
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0.0[0..self.len_bytes()]
    }

    pub const fn as_bytes(&self) -> &[u8; WIDTH] {
        &self.0.0
    }

    pub fn len(&self) -> usize {
        for i in (0..self.0.0.len()).rev() {
            if self.0.0[i] == 0 { continue; }

            // Found non-empty byte, scan forward, skipping the first char
            // It can't be empty as the byte isn't empty. Also, we only need 
            // to test 1 as a 5-bit encoding can't have more than 2 partial
            // bytes
            let idx_char = (i*8/5)+1;
            return idx_char + if self.at_raw(idx_char) == 0 { 0 } else { 1 };
        }
        0
    }

    pub const fn is_empty(&self) -> bool {
        self.0.0[0] == 0
    }

    pub const fn capacity() -> usize {
        Self::CAPACITY
    }

    pub const CAPACITY: usize = if WIDTH*8/5 <= 64 { WIDTH*8/5 } else { panic!("CompactStr cannot be more than 64 bytes in length") };
}

impl<const WIDTH: usize> CompactStr<WIDTH> {
    fn at_unchecked<T: From<u8>>(&self, index: usize) -> T {
        T::from(match self.at_raw(index) {
            1 => 95,
            a => 63+a,
        })
    }

    fn at_raw(&self, index: usize) -> u8 {
        self.0.get_bits(index*5, 5)
    }
}


// //////////////// //
// // Characters // //
// //////////////// //

impl<const WIDTH: usize> CompactStr<WIDTH> {

    pub fn iter<'a, T: From<u8>>(&'a self) -> CompactChars<'a, T, WIDTH> {
        CompactChars::<'a, T, WIDTH>::new(self)
    }

    pub fn chars(&self) -> CompactChars<'_, char, WIDTH> {
        self.iter()
    }

    pub fn ascii(&self) -> CompactChars<'_, u8, WIDTH> {
        self.iter()
    }

}

pub struct CompactChars<'a, T: From<u8>, const WIDTH: usize> {
    idx: usize,
    len: usize,
    owner: &'a CompactStr<WIDTH>,
    __: std::marker::PhantomData<T>,
}

impl<'a, T: From<u8>, const WIDTH: usize> CompactChars<'a, T, WIDTH> {
    pub fn new(owner: &'a CompactStr<WIDTH>) -> Self {
        Self{ idx: 0, len: owner.len(), owner, __: Default::default() }
    }
}

impl<'a, T: From<u8>, const WIDTH: usize> Iterator for CompactChars<'a, T, WIDTH> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.len {
            self.idx += 1;
            Some(self.owner.at_unchecked::<T>(self.idx - 1)) 
        } else { 
            None
        }
    }
}

// ////////// //
// // Util // //
// ////////// //

const fn const_min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}