use std::io::{Read, Seek, Result};

use crate::MemOverlay;


impl<R> Read for MemOverlay<R>
where
    R: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut bytes_read = 0;

        while bytes_read < buf.len() {
            let bytes = self.read_next_chunk(&mut buf[bytes_read..])?;
            if bytes == 0 {
                break;
            }
            bytes_read += bytes;
        }

        Ok(bytes_read)
    }
}
