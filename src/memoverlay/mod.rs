use std::io::{Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};

mod read;
mod seek;
mod write;
mod display;

pub use read::*;
pub use seek::*;
pub use write::*;
pub use display::*;

use crate::{Patch, PatchLayer};

/// Puts a writable layer of bytes over some byte stream
///
/// ```txt
/// +-----------------+------------+-----------------+-------+----------------------+-------+-----------------+
/// |    chunk 0      |  chunk 1   |    chunk 2      |   3   |       chunk 4        |   5   |     chunk 6     |
/// +-----------------+------------+-----------------+-------+----------------------+-------+-----------------+
///
///                                +-----------------+       +----------------------+
///                                +-----------------+       +----------------------+
///                   +-------------------+                           +---------------------+
///                   +-------------------+                           +---------------------+
/// +---------------------------------------------------------------------------------------------------------+
/// +---------------------------------------------------------------------------------------------------------+
/// ```
#[derive(Clone)]
pub struct MemOverlay<R: Read + Seek> {
    #[allow(dead_code)]
    base: R,
    base_len: u64,
    pos: u64,
    patch_layers: Vec<PatchLayer>,
}

impl<R> From<R> for MemOverlay<R>
where
    R: Read + Seek,
{
    fn from(base: R) -> Self {
        let mut base = base;
        let pos = base.stream_position().unwrap();

        base.seek(SeekFrom::End(0)).unwrap();
        let base_len = base.stream_position().unwrap();
        base.seek(SeekFrom::Start(pos)).unwrap();

        Self {
            base,
            base_len,
            pos,
            patch_layers: Default::default(),
        }
    }
}

impl<R> MemOverlay<R>
where
    R: Read + Seek,
{
    pub fn add_bytes_at(&mut self, offset: u64, bytes: impl AsRef<[u8]>) -> Result<usize> {
        let current_position = self.stream_position()?;
        self.seek(SeekFrom::Start(offset))?;
        let bytes = self.write(bytes.as_ref())?;
        self.seek(SeekFrom::Start(current_position))?;
        Ok(bytes)
    }

    pub fn last_overlay_position(&self) -> Option<u64> {
        // iterate through all layers and find the maximum offset of a last byte
        self.patch_layers
            .iter()
            .map(|layer| {
                // do a hard unwrap here, because every layer needs at least one patch,
                // which contains at least one byte
                layer
                    .iter_patches()
                    .rev()
                    .next()
                    .map(|patch| patch.last_byte_offset())
                    .unwrap()
            })
            .max()
    }

    pub fn last_base_position(&self) -> u64 {
        self.base_len - 1
    }

    pub fn last_valid_position(&self) -> u64 {
        std::cmp::max(
            self.last_overlay_position().unwrap_or(0),
            self.last_base_position(),
        )
    }

    fn set_new_position(&mut self, new_pos: u64) -> Result<u64> {
        //println!("set new position to {new_pos}");
        if new_pos > self.last_valid_position() {
            Err(Error::new(
                ErrorKind::UnexpectedEof,
                "cannot seek beyond end of file",
            ))
        } else {
            self.base.seek(SeekFrom::Start(new_pos))?;
            self.pos = new_pos;
            Ok(new_pos)
        }
    }

    fn shift_position(&mut self, bytes: usize) -> Result<()> {
        self.pos += match TryInto::<u64>::try_into(bytes) {
            Ok(bytes) => bytes,
            Err(_why) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    "read more bytes than can be displayed with a 64 bit counter",
                ))
            }
        };
        Ok(())
    }

    /// read the next chunk of data. This might be a part of a patch, or data
    /// from the base stream
    fn read_next_chunk(&mut self, buf: &mut [u8]) -> Result<usize> {
        let next_patches: Vec<_> = self
            .patch_layers
            .iter()
            .map(|layer| layer.next_patch_for(self.pos))
            .collect();

        let mut current: Option<&Patch> = None;
        let mut next: Option<&Patch> = None;
        for result in next_patches {
            match result {
                crate::PatchSearchResult::PatchFollows { next_patch } => {
                    if let Some(p) = next {
                        if next_patch.begin() < p.begin() {
                            next = Some(next_patch)
                        }
                    } else {
                        next = Some(next_patch)
                    }
                }
                crate::PatchSearchResult::CurrentPatchIsTheLastOne { current_patch } => {
                    // we are inside a patch, so we MUST read from it
                    if current.is_none() {
                        current = Some(current_patch);
                    }
                }
                crate::PatchSearchResult::CurrentPatchIsFollowedBy {
                    current_patch,
                    next_patch,
                } => {
                    // we are inside a patch, so we MUST read from it
                    if current.is_none() {
                        current = Some(current_patch);
                    }

                    // but we must also take a look at where the next patch is
                    if let Some(p) = next {
                        if next_patch.begin() < p.begin() {
                            next = Some(next_patch)
                        }
                    } else {
                        next = Some(next_patch)
                    }
                }
                _ => (),
            }
        }

        match current {
            Some(current_patch) => {
                let bytes = self.read_from_patch(current_patch, next, buf)?;
                self.shift_position(bytes)?;
                self.base
                    .seek(SeekFrom::Current(bytes.try_into().unwrap()))?;
                Ok(bytes)
            }
            None => {
                let bytes = match next {
                    Some(next_patch) => {
                        let length: usize = (next_patch.begin() - self.pos).try_into().unwrap();
                        self.base.read(&mut buf[0..length])?
                    }
                    None => self.base.read(buf)?,
                };
                self.shift_position(bytes)?;
                Ok(bytes)
            }
        }
    }

    fn read_from_patch(
        &self,
        current_patch: &Patch,
        next_patch: Option<&Patch>,
        buf: &mut [u8],
    ) -> Result<usize> {
        let offset = self.pos - current_patch.begin();
        let bytes = match next_patch {
            Some(next_patch) => {
                let length: usize = (next_patch.begin() - self.pos).try_into().unwrap();
                current_patch.read(offset, &mut buf[0..length])?
            }
            None => current_patch.read(offset, buf)?,
        };
        Ok(bytes)
    }
}
