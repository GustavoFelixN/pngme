use crate::chunk_type::ChunkType;
use crc;

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    message: Vec<u8>,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 12 {
            return Err("Not enough bytes to form a Chunk");
        }

        let length = u32::from_be_bytes(value[..4].try_into().unwrap());

        let mut type_bytes = [0u8; 4];
        type_bytes.clone_from_slice(&value[4..8]);
        let chunk_type = ChunkType::try_from(type_bytes)?;

        let message_end_index = 8 + length as usize;
        let message = value[8..message_end_index].to_vec();

        let crc = u32::from_be_bytes(value[message_end_index..].try_into().unwrap());

        Ok(Chunk {
            length,
            chunk_type,
            message,
            crc,
        })
    }
}

impl Chunk {
    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length: u32 = data.len() as u32;

        let bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();
        let crc_iso_3309 = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let crc_checksum = crc_iso_3309.checksum(&bytes);

        Chunk {
            length,
            chunk_type,
            message: data,
            crc: crc_checksum,
        }
    }

    fn length(&self) -> u32 {
        self.length
    }

    fn crc(&self) -> u32 {
        self.crc
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data_as_string(&self) -> Result<String, &'static str> {
        if self.message.len() == 0 {
            return Err("No data to show");
        }
        let message_str: String = self.message.iter().map(|&byte| byte as char).collect();
        Ok(message_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }
    //
    //     #[test]
    //     fn test_chunk_crc() {
    //         let chunk = testing_chunk();
    //         assert_eq!(chunk.crc(), 2882656334);
    //     }
    //
    //     #[test]
    //     fn test_valid_chunk_from_bytes() {
    //         let data_length: u32 = 42;
    //         let chunk_type = "RuSt".as_bytes();
    //         let message_bytes = "This is where your secret message will be!".as_bytes();
    //         let crc: u32 = 2882656334;
    //
    //         let chunk_data: Vec<u8> = data_length
    //             .to_be_bytes()
    //             .iter()
    //             .chain(chunk_type.iter())
    //             .chain(message_bytes.iter())
    //             .chain(crc.to_be_bytes().iter())
    //             .copied()
    //             .collect();
    //
    //         let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();
    //
    //         let chunk_string = chunk.data_as_string().unwrap();
    //         let expected_chunk_string = String::from("This is where your secret message will be!");
    //
    //         assert_eq!(chunk.length(), 42);
    //         assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    //         assert_eq!(chunk_string, expected_chunk_string);
    //         assert_eq!(chunk.crc(), 2882656334);
    //     }
    //
    //     #[test]
    //     fn test_invalid_chunk_from_bytes() {
    //         let data_length: u32 = 42;
    //         let chunk_type = "RuSt".as_bytes();
    //         let message_bytes = "This is where your secret message will be!".as_bytes();
    //         let crc: u32 = 2882656333;
    //
    //         let chunk_data: Vec<u8> = data_length
    //             .to_be_bytes()
    //             .iter()
    //             .chain(chunk_type.iter())
    //             .chain(message_bytes.iter())
    //             .chain(crc.to_be_bytes().iter())
    //             .copied()
    //             .collect();
    //
    //         let chunk = Chunk::try_from(chunk_data.as_ref());
    //
    //         assert!(chunk.is_err());
    //     }
    //
    //     #[test]
    //     pub fn test_chunk_trait_impls() {
    //         let data_length: u32 = 42;
    //         let chunk_type = "RuSt".as_bytes();
    //         let message_bytes = "This is where your secret message will be!".as_bytes();
    //         let crc: u32 = 2882656334;
    //
    //         let chunk_data: Vec<u8> = data_length
    //             .to_be_bytes()
    //             .iter()
    //             .chain(chunk_type.iter())
    //             .chain(message_bytes.iter())
    //             .chain(crc.to_be_bytes().iter())
    //             .copied()
    //             .collect();
    //
    //         let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
    //
    //         let _chunk_string = format!("{}", chunk);
    //     }
}
