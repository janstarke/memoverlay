use crate::MemOverlay;
use std::io::{Read, Seek};

use std::fmt::{Debug, Display};

impl<R> Display for MemOverlay<R>
where
    R: Read + Seek,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MemOverlay with {} layers and {} bytes of base content", self.patch_layers.len(), self.base_len)
    }
}

impl<R> Debug for MemOverlay<R>
where
    R: Read + Seek,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}