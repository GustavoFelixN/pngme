use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

const BIT_FIVE_MASK: u8 = 0b00100000;

#[derive(Debug)]
struct ChunkType {
    bytes: [u8; 4],
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            Err("String com tamanho incorreto")
        } else {
            let str_bytes = s.as_bytes();
            let mut array: [u8; 4] = [0; 4];
            array.copy_from_slice(&str_bytes);
            Ok(ChunkType { bytes: array })
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:08b}, {:08b}, {:08b}, {:08b}]",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]
        )
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for ChunkType {}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_critical(&self) -> bool {
        self.bytes()[0] & BIT_FIVE_MASK == 0
    }

    fn is_public(&self) -> bool {
        self.bytes()[1] & BIT_FIVE_MASK == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }
    //
    //     #[test]
    //     pub fn test_chunk_type_is_reserved_bit_valid() {
    //         let chunk = ChunkType::from_str("RuSt").unwrap();
    //         assert!(chunk.is_reserved_bit_valid());
    //     }
    //
    //     #[test]
    //     pub fn test_chunk_type_is_reserved_bit_invalid() {
    //         let chunk = ChunkType::from_str("Rust").unwrap();
    //         assert!(!chunk.is_reserved_bit_valid());
    //     }
    //
    //     #[test]
    //     pub fn test_chunk_type_is_safe_to_copy() {
    //         let chunk = ChunkType::from_str("RuSt").unwrap();
    //         assert!(chunk.is_safe_to_copy());
    //     }
    //
    //     #[test]
    //     pub fn test_chunk_type_is_unsafe_to_copy() {
    //         let chunk = ChunkType::from_str("RuST").unwrap();
    //         assert!(!chunk.is_safe_to_copy());
    //     }
    //
    //     #[test]
    //     pub fn test_valid_chunk_is_valid() {
    //         let chunk = ChunkType::from_str("RuSt").unwrap();
    //         assert!(chunk.is_valid());
    //     }
    //
    //     #[test]
    //     pub fn test_invalid_chunk_is_valid() {
    //         let chunk = ChunkType::from_str("Rust").unwrap();
    //         assert!(!chunk.is_valid());
    //
    //         let chunk = ChunkType::from_str("Ru1t");
    //         assert!(chunk.is_err());
    //     }
    //
    //     #[test]
    //     pub fn test_chunk_type_string() {
    //         let chunk = ChunkType::from_str("RuSt").unwrap();
    //         assert_eq!(&chunk.to_string(), "RuSt");
    //     }
    //
    //     #[test]
    //     pub fn test_chunk_type_trait_impls() {
    //         let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    //         let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    //         let _chunk_string = format!("{}", chunk_type_1);
    //         let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    //     }
}
