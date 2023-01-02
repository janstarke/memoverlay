use std::io::{Write, Read, Seek, ErrorKind, self};

use crate::{MemOverlay, Patch, SolidPatch, PatchLayer};

impl<R> Write for MemOverlay<R>
where
    R: Read + Seek,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let patch = match Patch::new(self.pos, buf) {
            Ok(patch) => patch,
            Err(err) => return Err(io::Error::new(ErrorKind::InvalidData, err)),
        };

        // find the first layer where this patch does not overlap with any other patch
        match self
            .patch_layers
            .iter_mut()
            .rev()
            .find(|layer| layer.may_contain(&patch))
        {
            Some(layer) => {
                layer.insert(patch);
            }
            None => {
                let layer = PatchLayer::new_with(patch);

                // insert at position 0 to make sure that a call to read()
                // always accesses the most recent patches first
                self.patch_layers.insert(0, layer);
            }
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // don't do anything
        Ok(())
    }
}
