extern crate bytes;
extern crate cobs;
extern crate tokio_io;

use tokio_io::codec::{Decoder, Encoder};
use bytes::{Bytes, BytesMut};

pub struct CobsCodec;

#[derive(Debug)]
pub enum CobsCodecError {
    IoError(std::io::Error),
    EncodingError,
}

impl std::convert::From<std::io::Error> for CobsCodecError {
    fn from(err: std::io::Error) -> CobsCodecError {
        CobsCodecError::IoError(err)
    }
}

impl std::convert::From<()> for CobsCodecError {
    fn from(_err: ()) -> CobsCodecError {
        CobsCodecError:: EncodingError
    }
}

impl Decoder for CobsCodec {
    type Item = Bytes;
    type Error = CobsCodecError;
    
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        for i in 0..src.len() {
            if src[i] == 0 {
                let mut frame = src.split_to(i + 1);
                
                let len = cobs::decode_in_place(&mut frame)?;
                frame.truncate(len);
                return Ok(Some(frame.freeze()));
            }
        }
        
        // nothing to see here folks, move along
        Ok(None)
    }
}

impl Encoder for CobsCodec {
    type Item = Bytes;
    type Error = std::io::Error;
    
    fn encode(&mut self, item: Self::Item, mut dst: &mut BytesMut) -> Result<(), Self::Error> {
        let max_len = cobs::max_encoding_length(item.len());
        let dst_len = dst.len();
        if dst_len < max_len {
            dst.reserve(max_len - dst_len);
        }
        
        let len = cobs::encode(&item, &mut dst);
        dst.truncate(len);
        
        Ok(())
    }
}