use std::{hash::Hash, io::{Cursor, Seek, SeekFrom, Read}};

use crate::{Contains, OverlayError, SolidPatch};

/// represents a memory patch. It is not allowed to create an empty patch
/// 
/// # Example
/// ```
/// use memoverlay::{SolidPatch, Patch};
/// assert!(Patch::new(10, &[0,1,2,3,4,5,6,7,8,9][..]).is_ok());
/// assert!(Patch::new(10, &[][..]).is_err());
/// assert!(Patch::new(10, &[0,1,2,3,4,5,6,7,8,9][3..3]).is_err());
/// ```
#[derive(Clone)]
pub struct Patch {
    offset: u64,
    content: Vec<u8>,
}

impl Patch {
    /// returns a unique id of this patch. Two patches have the same id if they
    /// have the same position and the same length. If two patches have the same
    /// offset, than the patch with the greater length has te greater id
    pub fn id(&self) -> u128 {
        let offset = (self.offset as u128) << 64;
        let len = TryInto::<u128>::try_into(self.content.len()).unwrap();
        offset | len
    }

    /// returns the offset of the first byte of this patch
    pub fn begin(&self) -> u64 {
        self.offset
    }

    /// returns the offset of the first byte after this patch
    pub fn end(&self) -> u64 {
        self.offset + TryInto::<u64>::try_into(self.content.len()).unwrap()
    }

    /// returns the offset of the first byte of this patch
    pub fn first_byte_offset(&self) -> u64 {
        self.offset
    }

    /// returns the offset of the last byte of this patch, or `None` if the patch
    /// has a length of *zero*
    pub fn last_byte_offset(&self) -> u64 {
        assert!(! self.content.is_empty());
        self.offset + TryInto::<u64>::try_into(self.content.len() - 1).unwrap()
    }

    /// checks if two patches overlap each other
    ///
    /// # Example
    /// ```
    /// use memoverlay::{SolidPatch, Patch};
    ///
    /// let patch1 = Patch::new(10, &[0,1,2,3,4,5,6,7,8,9][..]).unwrap();
    /// assert!(patch1.overlaps(&patch1));
    ///
    /// let patch2 = Patch::new(12, &[1,2,3][..]).unwrap();
    /// assert!(patch1.overlaps(&patch2));
    /// assert!(patch2.overlaps(&patch1));
    ///
    /// let patch3 = Patch::new(15, &[1,2,3,4,5][..]).unwrap();
    /// assert!(patch1.overlaps(&patch3));
    /// assert!(! patch2.overlaps(&patch3));
    ///
    /// let patch4 = Patch::new(0, &[0,1,2,3,4,5,6,7,8,9][..]).unwrap();
    /// assert!(! patch1.overlaps(&patch4));
    /// ```
    pub fn overlaps(&self, other: &Self) -> bool {
        other.contains(self.first_byte_offset()) || self.contains(other.first_byte_offset())
    }

    pub fn read(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut cursor = Cursor::new(&self.content);
        cursor.seek(SeekFrom::Start(offset))?;
        cursor.read(buf)
    }
}

impl Hash for Patch {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for Patch {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl PartialOrd for Patch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

impl Ord for Patch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id().cmp(&other.id())
    }
}

impl Eq for Patch {}

impl Contains for Patch {
    fn contains(&self, offset: u64) -> bool {
        self.begin() <= offset && offset < self.end()
    }
}

impl SolidPatch<&[u8]> for Patch {
    fn new(offset: u64, content: &[u8]) -> Result<Self, OverlayError> {
        if content.is_empty() {
            Err(OverlayError::EmptyPatch)
        } else {
            Ok(Self {
                offset,
                content: Vec::from(content),
            })
        }
    }
}

impl SolidPatch<Vec<u8>> for Patch {
    fn new(offset: u64, content: Vec<u8>) -> Result<Self, OverlayError> {
        if content.is_empty() {
            Err(OverlayError::EmptyPatch)
        } else {
            Ok(Self { offset, content })
        }
    }
}
