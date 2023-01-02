use std::io::{ErrorKind, Error, Seek, Read};

use crate::MemOverlay;


fn checked_add(a: u64, b: i64) -> std::io::Result<u64> {
    if let Ok(pos) = TryInto::<i64>::try_into(a) {
        if let Ok(new_pos) = TryInto::<u64>::try_into(pos + b) {
            Ok(new_pos)
        } else {
            Err(Error::new(
                ErrorKind::UnexpectedEof,
                "cannot seek before start of file",
            ))
        }
    } else if let Ok(diff) = TryInto::<u64>::try_into(b) {
        if let Ok(new_pos) = TryInto::<u64>::try_into(a + diff) {
            Ok(new_pos)
        } else {
            Err(Error::new(
                ErrorKind::UnexpectedEof,
                "cannot seek beyond of file",
            ))
        }
    } else {
        Err(Error::new(
            ErrorKind::UnexpectedEof,
            "cannot calculate new file position",
        ))
    }
}


impl<R> Seek for MemOverlay<R>
where
    R: Read + Seek,
{
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        //println!("seeking to {pos:?}");
        match pos {
            std::io::SeekFrom::Start(diff) => self.set_new_position(diff),

            std::io::SeekFrom::End(diff) => {
                self.set_new_position(checked_add(self.last_valid_position(), diff)?)
            }

            std::io::SeekFrom::Current(diff) => self.set_new_position(checked_add(self.pos, diff)?),
        }
    }
}