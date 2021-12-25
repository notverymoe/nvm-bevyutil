/*========================================================================*\
** NotVeryMoe BevyUtil | Copyright 2021 NotVeryMoe (projects@notvery.moe) **
\*========================================================================*/

use super::*;

#[test]
fn empty() {
    let v = CompactStr::<1>::try_parse_str("").unwrap();
    assert_eq!(v.len(),         0);
    assert_eq!(v.as_bytes()[0], 0); 
}

#[test]
fn overflow() {
    assert!(CompactStr::<1>::try_parse_str(""   ).is_ok());
    assert!(CompactStr::<1>::try_parse_str("a"  ).is_ok());
    assert!(CompactStr::<1>::try_parse_str("aa" ).is_err());
    assert!(CompactStr::<1>::try_parse_str("aaa").is_err());
}

#[test]
fn chars() {
    fn character(ch: char) {
        let v = CompactStr::<1>::try_parse_str(&ch.to_string()).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v.as_bytes()[0], match ch {
            'a'..='z' | 'A'..='Z' => (ch.to_ascii_lowercase() as u8)-63,
            _                     => 1
        } << 3);
    }

    let chars = "_abcdefghijklmnopqrstuvwxyz ";
    chars.chars()
        .chain(chars.to_uppercase().chars())
        .for_each(character);
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn short() {
    let str = "abc";
    let v = CompactStr::<2>::try_parse_str(str).unwrap();
    let str = sanitize_str(str);

    assert_eq!(v.len(),    str.len());
    assert_eq!(v.to_str(), str);

    // 5-bit groupings across byte boundaries
    for (i, byte) in [
        0b00010_000, 0b11_00100_0
    ].iter().enumerate() {
        assert_eq!(v.as_bytes()[i], *byte); 
    }
}

#[test]
#[allow(clippy::unusual_byte_groupings)]
fn long() {
    let str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ_!@#$%^&*()_+[]:\"';?></.,1234567890";
    let v = CompactStr::<48>::try_parse_str(&str.to_ascii_lowercase()).unwrap();
    let str = sanitize_str(str);

    assert_eq!(v.len(),    str.len());
    assert_eq!(v.to_str(), str      );

    // 5-bit groupings across byte boundaries
    for (i, byte) in [
        0b00010_000, 0b11_00100_0, 0b0101_0011, 0b0_00111_01, 0b000_01001,
        0b01010_010, 0b11_01100_0, 0b1101_0111, 0b0_01111_10, 0b000_10001,
        0b10010_100, 0b11_10100_1, 0b0101_1011, 0b0_10111_11, 0b000_11001,
        0b11010_110, 0b11_00001_0, 0b0001_0000, 0b1_00001_00, 0b001_00001,
        0b00001_000, 0b01_00001_0, 0b0001_0000, 0b1_00001_00, 0b001_00001,
        0b00001_000, 0b01_00001_0, 0b0001_0000, 0b1_00001_00, 0b001_00001,
        0b00001_000, 0b01_00001_0, 0b0001_0000, 0b1_00001_00, 0b001_00001,
        0b00001_000, 0b01_00001_0, 0b0001_0000, 0b1_00000_00,
    ].iter().enumerate() {
        assert_eq!(v.as_bytes()[i], *byte); 
    }
}


fn sanitize_str(s: &str) -> String {
    s.chars().map(|c| match c {
        'a'..='z' | 'A'..='Z' => c.to_ascii_uppercase(),
        _ => '_',
    }).collect()
}